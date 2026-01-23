//! Axum server utilities for stage microservices.
//! 
//! This module provides the common server infrastructure that all stage
//! microservices share: routing, health checks, job submission, etc.

use crate::microservice::{
    HealthResponse, JobProgress, JobStatusResponse, StageJobRequest, StageJobResult, StageNumber,
};
use anyhow::Result;
use async_trait::async_trait;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use chrono::Utc;
use std::{
    collections::HashMap,
    sync::Arc,
    time::Instant,
};
use tokio::sync::RwLock;
use tracing::{error, info};

// ============================================================================
// STAGE SERVICE TRAIT
// ============================================================================

/// Trait that all stage processors must implement.
/// 
/// This defines the core processing logic for a stage. The server infrastructure
/// handles HTTP endpoints, job queuing, health checks, etc.
#[async_trait]
pub trait StageProcessor: Send + Sync + 'static {
    /// Which stage this processor handles.
    fn stage(&self) -> StageNumber;
    
    /// Human-readable service name (e.g., "igait-stage1-media-conversion").
    fn service_name(&self) -> &'static str;
    
    /// Service version.
    fn version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
    
    /// Process a job request.
    /// 
    /// Implementations should:
    /// 1. Download input files from storage
    /// 2. Process them
    /// 3. Upload output files to storage
    /// 4. Return a StageJobResult
    async fn process(&self, request: StageJobRequest) -> StageJobResult;
}

// ============================================================================
// SERVER STATE
// ============================================================================

/// Shared state for the stage server.
pub struct StageServerState<P: StageProcessor> {
    /// The stage processor implementation
    processor: Arc<P>,
    
    /// Currently processing jobs
    processing_jobs: RwLock<HashMap<String, JobState>>,
    
    /// Callback client for notifying backend
    callback_client: reqwest::Client,
}

/// State of a job being processed.
struct JobState {
    received_at: chrono::DateTime<Utc>,
    started_at: Option<chrono::DateTime<Utc>>,
    progress: JobProgress,
}

impl<P: StageProcessor> StageServerState<P> {
    pub fn new(processor: P) -> Self {
        Self {
            processor: Arc::new(processor),
            processing_jobs: RwLock::new(HashMap::new()),
            callback_client: reqwest::Client::new(),
        }
    }
}

// ============================================================================
// ROUTE HANDLERS
// ============================================================================

/// POST /submit - Submit a job for processing
async fn submit_job<P: StageProcessor>(
    State(state): State<Arc<StageServerState<P>>>,
    Json(request): Json<StageJobRequest>,
) -> impl IntoResponse {
    let job_id = request.job_id.clone();
    let callback_url = request.callback_url.clone();
    
    info!("Received job submission: {}", job_id);
    
    // Check if job is already being processed
    {
        let jobs = state.processing_jobs.read().await;
        if jobs.contains_key(&job_id) {
            return (
                StatusCode::CONFLICT,
                Json(serde_json::json!({
                    "error": "Job already being processed",
                    "job_id": job_id
                })),
            );
        }
    }
    
    // Add job to processing set
    {
        let mut jobs = state.processing_jobs.write().await;
        jobs.insert(
            job_id.clone(),
            JobState {
                received_at: Utc::now(),
                started_at: Some(Utc::now()),
                progress: JobProgress::Processing,
            },
        );
    }
    
    // Spawn processing task
    let state_clone = Arc::clone(&state);
    let request_clone = request.clone();
    
    tokio::spawn(async move {
        let start_time = Instant::now();
        
        // Process the job
        let result = state_clone.processor.process(request_clone).await;
        let duration = start_time.elapsed();
        
        info!(
            "Job {} completed with status {:?} in {:?}",
            result.job_id, result.status, duration
        );
        
        // Remove from processing set
        {
            let mut jobs = state_clone.processing_jobs.write().await;
            jobs.remove(&result.job_id);
        }
        
        // Send callback to backend
        if let Err(e) = state_clone
            .callback_client
            .post(&callback_url)
            .json(&result)
            .send()
            .await
        {
            error!("Failed to send callback for job {}: {}", result.job_id, e);
        } else {
            info!("Sent callback for job {}", result.job_id);
        }
    });
    
    (
        StatusCode::ACCEPTED,
        Json(serde_json::json!({
            "message": "Job accepted for processing",
            "job_id": job_id
        })),
    )
}

/// GET /health - Health check endpoint
async fn health_check<P: StageProcessor>(
    State(state): State<Arc<StageServerState<P>>>,
) -> Json<HealthResponse> {
    let jobs = state.processing_jobs.read().await;
    let processing_count = jobs
        .values()
        .filter(|j| j.progress == JobProgress::Processing)
        .count();
    let queued_count = jobs
        .values()
        .filter(|j| j.progress == JobProgress::Queued)
        .count();
    
    Json(HealthResponse {
        healthy: true,
        service: state.processor.service_name().to_string(),
        stage: state.processor.stage(),
        version: state.processor.version().to_string(),
        timestamp: Utc::now(),
        jobs_processing: processing_count,
        jobs_queued: queued_count,
    })
}

/// GET /jobs/:job_id - Get job status
async fn get_job_status<P: StageProcessor>(
    State(state): State<Arc<StageServerState<P>>>,
    Path(job_id): Path<String>,
) -> Json<JobStatusResponse> {
    let jobs = state.processing_jobs.read().await;
    
    if let Some(job) = jobs.get(&job_id) {
        Json(JobStatusResponse {
            job_id,
            status: job.progress,
            received_at: job.received_at,
            started_at: job.started_at,
            progress_percent: None,
        })
    } else {
        Json(JobStatusResponse {
            job_id,
            status: JobProgress::NotFound,
            received_at: Utc::now(),
            started_at: None,
            progress_percent: None,
        })
    }
}

// ============================================================================
// SERVER BUILDER
// ============================================================================

/// Builds and runs a stage microservice server.
pub struct StageServer<P: StageProcessor> {
    processor: P,
    port: u16,
}

impl<P: StageProcessor> StageServer<P> {
    /// Creates a new stage server with the given processor.
    pub fn new(processor: P) -> Self {
        Self {
            processor,
            port: 8080,
        }
    }
    
    /// Sets the port to listen on (default: 8080).
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }
    
    /// Builds the Axum router.
    pub fn build_router(self) -> Router {
        let state = Arc::new(StageServerState::new(self.processor));
        
        Router::new()
            .route("/submit", post(submit_job::<P>))
            .route("/health", get(health_check::<P>))
            .route("/jobs/:job_id", get(get_job_status::<P>))
            .with_state(state)
    }
    
    /// Runs the server.
    pub async fn run(self) -> Result<()> {
        let port = self.port;
        let router = self.build_router();
        
        let addr = format!("0.0.0.0:{}", port);
        info!("Starting stage server on {}", addr);
        
        let listener = tokio::net::TcpListener::bind(&addr).await?;
        axum::serve(listener, router).await?;
        
        Ok(())
    }
}

/// Convenience macro to create the main function for a stage service.
/// 
/// # Example
/// 
/// ```ignore
/// use igait_lib::microservice::{stage_main, StageProcessor, StageNumber, StageJobRequest, StageJobResult};
/// 
/// struct MyProcessor;
/// 
/// #[async_trait::async_trait]
/// impl StageProcessor for MyProcessor {
///     fn stage(&self) -> StageNumber { StageNumber::Stage1MediaConversion }
///     fn service_name(&self) -> &'static str { "stage1-service" }
///     async fn process(&self, req: StageJobRequest) -> StageJobResult {
///         // ... processing logic
///     }
/// }
/// 
/// stage_main!(MyProcessor);
/// ```
#[macro_export]
macro_rules! stage_main {
    ($processor:expr) => {
        #[tokio::main]
        async fn main() -> anyhow::Result<()> {
            // Initialize tracing
            tracing_subscriber::fmt()
                .with_env_filter(
                    tracing_subscriber::EnvFilter::from_default_env()
                        .add_directive(tracing::Level::INFO.into()),
                )
                .init();

            let port = std::env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8080);

            $crate::microservice::StageServer::new($processor)
                .port(port)
                .run()
                .await
        }
    };
}
