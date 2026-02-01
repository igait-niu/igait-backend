//! Queue-based worker infrastructure for stage microservices.
//!
//! This module provides the worker loop that polls Firebase Realtime Database
//! queues, claims jobs using transactions, and processes them independently.

use crate::microservice::{
    queue::{
        ClaimResult, FinalizeQueueItem, ProcessingResult, QueueItem,
        CLAIM_TIMEOUT_MS, HEARTBEAT_INTERVAL_SECS,
        generate_worker_id, next_stage, now_ms, queue_item_path, queue_path,
    },
    StageNumber,
};
use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, sync::Arc, time::Duration};

// ============================================================================
// FIREBASE RTDB CLIENT
// ============================================================================

/// A simple Firebase Realtime Database client for queue operations.
/// 
/// This client supports the transaction-like pattern needed for safe job claiming.
#[derive(Clone)]
pub struct FirebaseRtdb {
    /// Base URL of the Firebase RTDB (e.g., "https://project-id.firebaseio.com")
    base_url: String,
    
    /// Auth token for database access
    auth_token: String,
    
    /// HTTP client
    client: Client,
}

impl FirebaseRtdb {
    /// Creates a new Firebase RTDB client.
    pub fn new(base_url: &str, auth_token: &str) -> Self {
        // Remove trailing slash if present
        let base_url = base_url.trim_end_matches('/').to_string();
        
        Self {
            base_url,
            auth_token: auth_token.to_string(),
            client: Client::new(),
        }
    }

    /// Creates a client from environment variables.
    /// 
    /// Expects:
    /// - `FIREBASE_RTDB_URL`: The database URL
    /// - `FIREBASE_ACCESS_KEY`: The auth token
    pub fn from_env() -> Result<Self> {
        let base_url = std::env::var("FIREBASE_RTDB_URL")
            .or_else(|_| Ok::<_, std::env::VarError>(
                "https://network-technology-project-default-rtdb.firebaseio.com".to_string()
            ))?;
        let auth_token = std::env::var("FIREBASE_ACCESS_KEY")
            .context("Missing FIREBASE_ACCESS_KEY environment variable")?;
        
        Ok(Self::new(&base_url, &auth_token))
    }

    /// Builds a URL for a given path.
    fn url(&self, path: &str) -> String {
        format!("{}/{}.json?auth={}", self.base_url, path, self.auth_token)
    }

    /// Gets data at a path.
    pub async fn get<T: for<'de> Deserialize<'de>>(&self, path: &str) -> Result<Option<T>> {
        let url = self.url(path);
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("Firebase GET failed ({}): {}", status, body);
        }
        
        let value: Value = response.json().await?;
        if value.is_null() {
            return Ok(None);
        }
        
        let data: T = serde_json::from_value(value)?;
        Ok(Some(data))
    }

    /// Sets data at a path (overwrites).
    pub async fn set<T: Serialize>(&self, path: &str, data: &T) -> Result<()> {
        let url = self.url(path);
        let response = self.client.put(&url).json(data).send().await?;
        
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("Firebase SET failed ({}): {}", status, body);
        }
        
        Ok(())
    }

    /// Updates specific fields at a path (PATCH).
    pub async fn update<T: Serialize>(&self, path: &str, data: &T) -> Result<()> {
        let url = self.url(path);
        let response = self.client.patch(&url).json(data).send().await?;
        
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("Firebase UPDATE failed ({}): {}", status, body);
        }
        
        Ok(())
    }

    /// Deletes data at a path.
    pub async fn delete(&self, path: &str) -> Result<()> {
        let url = self.url(path);
        let response = self.client.delete(&url).send().await?;
        
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("Firebase DELETE failed ({}): {}", status, body);
        }
        
        Ok(())
    }

    /// Performs a multi-path update (atomic update to multiple paths).
    /// 
    /// The updates map should have paths as keys (without leading slash)
    /// and the new values. Use `Value::Null` to delete a path.
    pub async fn multi_update(&self, updates: HashMap<String, Value>) -> Result<()> {
        let url = self.url("");
        let response = self.client.patch(&url).json(&updates).send().await?;
        
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("Firebase MULTI_UPDATE failed ({}): {}", status, body);
        }
        
        Ok(())
    }
}

// ============================================================================
// QUEUE OPERATIONS
// ============================================================================

/// Operations for working with stage queues.
pub struct QueueOps {
    db: FirebaseRtdb,
    worker_id: String,
}

impl QueueOps {
    /// Creates a new QueueOps instance.
    pub fn new(db: FirebaseRtdb, worker_id: String) -> Self {
        Self { db, worker_id }
    }

    /// Attempts to claim an available job from the specified stage queue.
    /// 
    /// This uses a read-then-write pattern with validation to minimize race conditions.
    /// If another worker claims the job between read and write, the write will
    /// effectively be a no-op (job will be re-processed due to timeout if the
    /// other worker fails).
    pub async fn claim_job(&self, stage: StageNumber) -> ClaimResult<QueueItem> {
        let path = queue_path(stage);
        
        // Read all items in the queue
        let items: Option<HashMap<String, QueueItem>> = match self.db.get(&path).await {
            Ok(items) => items,
            Err(e) => return ClaimResult::Error(format!("Failed to read queue: {}", e)),
        };

        let Some(items) = items else {
            return ClaimResult::QueueEmpty;
        };

        if items.is_empty() {
            return ClaimResult::QueueEmpty;
        }

        // Find an available item (unclaimed or stale)
        let now = now_ms();
        let mut available_item: Option<(String, QueueItem)> = None;

        for (key, item) in items {
            let is_unclaimed = item.claimed_by.is_none();
            let is_stale = item.claimed_at
                .map(|t| now.saturating_sub(t) > CLAIM_TIMEOUT_MS)
                .unwrap_or(false);

            if is_unclaimed || is_stale {
                available_item = Some((key, item));
                break;
            }
        }

        let Some((key, item)) = available_item else {
            return ClaimResult::AllClaimed;
        };

        // Claim the item
        let claimed_item = item.claim(&self.worker_id);
        let item_path = format!("{}/{}", path, key);

        if let Err(e) = self.db.set(&item_path, &claimed_item).await {
            return ClaimResult::Error(format!("Failed to claim job: {}", e));
        }

        ClaimResult::Claimed(claimed_item)
    }

    /// Updates the heartbeat for a claimed job to prevent timeout.
    pub async fn heartbeat(&self, stage: StageNumber, job_id: &str) -> Result<()> {
        let path = queue_item_path(stage, job_id);
        
        #[derive(Serialize)]
        struct HeartbeatUpdate {
            claimed_at: u64,
        }
        
        self.db.update(&path, &HeartbeatUpdate { claimed_at: now_ms() }).await
    }

    /// Moves a job to the next stage queue after successful processing.
    pub async fn move_to_next_stage(
        &self,
        current_stage: StageNumber,
        job: &QueueItem,
        output_keys: HashMap<String, String>,
    ) -> Result<()> {
        let next = next_stage(current_stage);
        
        // Create the item for the next queue
        let next_item = QueueItem::new(
            job.job_id.clone(),
            job.user_id.clone(),
            output_keys,
            job.metadata.clone(),
        );

        // Build multi-path update: delete from current, add to next
        let current_path = queue_item_path(current_stage, &job.job_id);
        let next_path = queue_item_path(next, &job.job_id);

        let mut updates = HashMap::new();
        updates.insert(current_path, Value::Null); // Delete from current
        updates.insert(next_path, serde_json::to_value(&next_item)?); // Add to next

        self.db.multi_update(updates).await?;
        
        Ok(())
    }

    /// Moves a job to the finalize queue after successful pipeline completion.
    pub async fn move_to_finalize_success(
        &self,
        current_stage: StageNumber,
        job: &QueueItem,
        output_keys: HashMap<String, String>,
    ) -> Result<()> {
        let finalize_item = FinalizeQueueItem::success(
            job.job_id.clone(),
            job.user_id.clone(),
            output_keys,
            job.metadata.clone(),
        );

        let current_path = queue_item_path(current_stage, &job.job_id);
        let finalize_path = queue_item_path(StageNumber::Stage7Finalize, &job.job_id);

        let mut updates = HashMap::new();
        updates.insert(current_path, Value::Null);
        updates.insert(finalize_path, serde_json::to_value(&finalize_item)?);

        self.db.multi_update(updates).await?;
        
        Ok(())
    }

    /// Moves a job to the finalize queue after a stage failure.
    pub async fn move_to_finalize_failure(
        &self,
        current_stage: StageNumber,
        job: &QueueItem,
        error: String,
        error_logs: Option<String>,
    ) -> Result<()> {
        let finalize_item = FinalizeQueueItem::failure(
            job.job_id.clone(),
            job.user_id.clone(),
            current_stage.as_u8(),
            error,
            error_logs,
            job.metadata.clone(),
        );

        let current_path = queue_item_path(current_stage, &job.job_id);
        let finalize_path = queue_item_path(StageNumber::Stage7Finalize, &job.job_id);

        let mut updates = HashMap::new();
        updates.insert(current_path, Value::Null);
        updates.insert(finalize_path, serde_json::to_value(&finalize_item)?);

        self.db.multi_update(updates).await?;
        
        Ok(())
    }

    /// Claims a job from the finalize queue.
    pub async fn claim_finalize_job(&self) -> ClaimResult<FinalizeQueueItem> {
        let path = queue_path(StageNumber::Stage7Finalize);
        
        let items: Option<HashMap<String, FinalizeQueueItem>> = match self.db.get(&path).await {
            Ok(items) => items,
            Err(e) => return ClaimResult::Error(format!("Failed to read finalize queue: {}", e)),
        };

        let Some(items) = items else {
            return ClaimResult::QueueEmpty;
        };

        if items.is_empty() {
            return ClaimResult::QueueEmpty;
        }

        let now = now_ms();
        let mut available_item: Option<(String, FinalizeQueueItem)> = None;

        for (key, item) in items {
            let is_unclaimed = item.claimed_by.is_none();
            let is_stale = item.claimed_at
                .map(|t| now.saturating_sub(t) > CLAIM_TIMEOUT_MS)
                .unwrap_or(false);

            if is_unclaimed || is_stale {
                available_item = Some((key, item));
                break;
            }
        }

        let Some((key, item)) = available_item else {
            return ClaimResult::AllClaimed;
        };

        // Claim it
        let claimed_item = FinalizeQueueItem {
            claimed_by: Some(self.worker_id.clone()),
            claimed_at: Some(now_ms()),
            ..item
        };
        
        let item_path = format!("{}/{}", path, key);
        if let Err(e) = self.db.set(&item_path, &claimed_item).await {
            return ClaimResult::Error(format!("Failed to claim finalize job: {}", e));
        }

        ClaimResult::Claimed(claimed_item)
    }

    /// Removes a completed job from the finalize queue.
    pub async fn complete_finalize(&self, job_id: &str) -> Result<()> {
        let path = queue_item_path(StageNumber::Stage7Finalize, job_id);
        self.db.delete(&path).await
    }
}

// ============================================================================
// STAGE WORKER TRAIT
// ============================================================================

/// Trait that stage workers must implement.
/// 
/// This is similar to the old `StageProcessor` but designed for queue-based operation.
#[async_trait]
pub trait StageWorker: Send + Sync + 'static {
    /// Which stage this worker handles.
    fn stage(&self) -> StageNumber;
    
    /// Human-readable service name.
    fn service_name(&self) -> &'static str;
    
    /// Process a job from the queue.
    /// 
    /// Returns `ProcessingResult::Success` with output keys on success,
    /// or `ProcessingResult::Failure` with error info on failure.
    async fn process(&self, job: &QueueItem) -> ProcessingResult;
}

// ============================================================================
// WORKER RUNNER
// ============================================================================

/// Configuration for the worker runner.
#[derive(Clone)]
pub struct WorkerConfig {
    /// How long to wait between queue polls when no jobs are available
    pub poll_interval: Duration,
    
    /// How long to wait after an error before retrying
    pub error_backoff: Duration,
    
    /// Whether to keep running after a fatal error (vs. crashing)
    pub resilient: bool,
}

impl Default for WorkerConfig {
    fn default() -> Self {
        Self {
            poll_interval: Duration::from_secs(5),
            error_backoff: Duration::from_secs(10),
            resilient: true,
        }
    }
}

/// Runs a stage worker in a continuous loop.
pub struct WorkerRunner<W: StageWorker> {
    worker: Arc<W>,
    queue_ops: QueueOps,
    config: WorkerConfig,
    worker_id: String,
}

impl<W: StageWorker> WorkerRunner<W> {
    /// Creates a new worker runner.
    pub fn new(worker: W, db: FirebaseRtdb) -> Self {
        let worker_id = generate_worker_id(worker.service_name());
        let queue_ops = QueueOps::new(db, worker_id.clone());
        
        Self {
            worker: Arc::new(worker),
            queue_ops,
            config: WorkerConfig::default(),
            worker_id,
        }
    }

    /// Sets custom configuration.
    pub fn with_config(mut self, config: WorkerConfig) -> Self {
        self.config = config;
        self
    }

    /// Runs the worker loop.
    /// 
    /// This will continuously:
    /// 1. Poll the queue for available jobs
    /// 2. Claim and process any available job
    /// 3. Move the job to the next queue (or finalize queue on failure)
    /// 4. Sleep if no jobs are available
    pub async fn run(&self) -> Result<()> {
        let stage = self.worker.stage();
        println!(
            "[{}] Starting worker {} for stage {} ({})",
            self.worker_id,
            self.worker.service_name(),
            stage.as_u8(),
            stage.name()
        );

        loop {
            match self.process_one_job().await {
                Ok(true) => {
                    // Processed a job, immediately check for more
                    continue;
                }
                Ok(false) => {
                    // No jobs available, wait before polling again
                    tokio::time::sleep(self.config.poll_interval).await;
                }
                Err(e) => {
                    eprintln!("[{}] Error in worker loop: {:?}", self.worker_id, e);
                    
                    if self.config.resilient {
                        tokio::time::sleep(self.config.error_backoff).await;
                    } else {
                        return Err(e);
                    }
                }
            }
        }
    }

    /// Attempts to process one job from the queue.
    /// 
    /// Returns `Ok(true)` if a job was processed, `Ok(false)` if no jobs were available.
    async fn process_one_job(&self) -> Result<bool> {
        let stage = self.worker.stage();

        // Try to claim a job
        let job = match self.queue_ops.claim_job(stage).await {
            ClaimResult::Claimed(job) => job,
            ClaimResult::QueueEmpty | ClaimResult::AllClaimed => {
                return Ok(false);
            }
            ClaimResult::Error(e) => {
                anyhow::bail!("Failed to claim job: {}", e);
            }
        };

        println!(
            "[{}] Claimed job {} for processing",
            self.worker_id, job.job_id
        );

        // Spawn heartbeat task for long-running jobs
        let heartbeat_db = self.queue_ops.db.clone();
        let heartbeat_worker_id = self.worker_id.clone();
        let heartbeat_job_id = job.job_id.clone();
        let heartbeat_stage = stage;
        
        let heartbeat_handle = tokio::spawn(async move {
            let ops = QueueOps::new(heartbeat_db, heartbeat_worker_id);
            loop {
                tokio::time::sleep(Duration::from_secs(HEARTBEAT_INTERVAL_SECS)).await;
                if let Err(e) = ops.heartbeat(heartbeat_stage, &heartbeat_job_id).await {
                    eprintln!("Heartbeat failed: {:?}", e);
                    break;
                }
            }
        });

        // Process the job
        let result = self.worker.process(&job).await;

        // Cancel heartbeat
        heartbeat_handle.abort();

        // Handle result
        match result {
            ProcessingResult::Success { output_keys, logs: _, duration_ms } => {
                println!(
                    "[{}] Job {} completed successfully in {}ms",
                    self.worker_id, job.job_id, duration_ms
                );

                // Check if this is the last processing stage (stage 6)
                // Stage 7 is finalize, so stage 6 sends to finalize on success
                if stage == StageNumber::Stage6Prediction {
                    self.queue_ops
                        .move_to_finalize_success(stage, &job, output_keys)
                        .await
                        .context("Failed to move job to finalize queue")?;
                } else {
                    self.queue_ops
                        .move_to_next_stage(stage, &job, output_keys)
                        .await
                        .context("Failed to move job to next stage")?;
                }
            }
            ProcessingResult::Failure { error, logs, duration_ms } => {
                eprintln!(
                    "[{}] Job {} failed after {}ms: {}",
                    self.worker_id, job.job_id, duration_ms, error
                );

                self.queue_ops
                    .move_to_finalize_failure(stage, &job, error, Some(logs))
                    .await
                    .context("Failed to move job to finalize queue")?;
            }
        }

        Ok(true)
    }
}

// ============================================================================
// CONVENIENCE FUNCTION
// ============================================================================

/// Runs a stage worker with default configuration.
/// 
/// This is the main entry point for stage microservices.
/// 
/// # Example
/// 
/// ```ignore
/// use igait_lib::microservice::{run_stage_worker, StageWorker, StageNumber, QueueItem, ProcessingResult};
/// 
/// struct MyStageWorker;
/// 
/// #[async_trait::async_trait]
/// impl StageWorker for MyStageWorker {
///     fn stage(&self) -> StageNumber { StageNumber::Stage2ValidityCheck }
///     fn service_name(&self) -> &'static str { "stage2-validity-check" }
///     
///     async fn process(&self, job: &QueueItem) -> ProcessingResult {
///         // ... do work ...
///         ProcessingResult::Success {
///             output_keys: job.input_keys.clone(),
///             logs: "Done!".to_string(),
///             duration_ms: 100,
///         }
///     }
/// }
/// 
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     run_stage_worker(MyStageWorker).await
/// }
/// ```
pub async fn run_stage_worker<W: StageWorker>(worker: W) -> Result<()> {
    let db = FirebaseRtdb::from_env()?;
    let runner = WorkerRunner::new(worker, db);
    runner.run().await
}

/// Convenience macro for creating the main function of a stage worker.
#[macro_export]
macro_rules! stage_worker_main {
    ($worker:expr) => {
        #[tokio::main]
        async fn main() -> anyhow::Result<()> {
            $crate::microservice::run_stage_worker($worker).await
        }
    };
}
