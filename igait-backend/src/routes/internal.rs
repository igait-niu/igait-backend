//! Internal endpoint for microservice status updates
//!
//! This endpoint is NOT exposed publicly and should only be called by
//! the stage microservices to update job status.

use axum::{
    extract::State,
    Json,
};
use serde::{Deserialize, Serialize};
use anyhow::Context;

use igait_lib::microservice::{FirebaseRtdb, QueueItem, StageNumber, queue_path};

use crate::helper::{
    lib::{AppError, AppStatePtr, JobStatus},
};

/// Request body for status update
#[derive(Debug, Deserialize)]
pub struct UpdateStatusRequest {
    pub user_id: String,
    pub job_index: usize,
    pub status: JobStatus,
}

/// Response for status update
#[derive(Debug, Serialize)]
pub struct UpdateStatusResponse {
    pub success: bool,
}

/// Internal endpoint to update job status
/// 
/// This is called by stage microservices to update the status of a job
/// as it progresses through the pipeline.
pub async fn update_status(
    State(app): State<AppStatePtr>,
    Json(request): Json<UpdateStatusRequest>,
) -> Result<Json<UpdateStatusResponse>, AppError> {
    println!(
        "Received status update for user {} job {}: {:?}",
        request.user_id, request.job_index, request.status.code()
    );

    // Update the status in the database
    app.state
        .db
        .lock()
        .await
        .update_status(&request.user_id, request.job_index, request.status)
        .await
        .context("Failed to update job status")?;

    Ok(Json(UpdateStatusResponse { success: true }))
}

// ============================================================================
// APPROVE JOB ENDPOINT
// ============================================================================

/// Request body for approving a job
#[derive(Debug, Deserialize)]
pub struct ApproveJobRequest {
    pub user_id: String,
    pub job_index: usize,
}

/// Response for job approval
#[derive(Debug, Serialize)]
pub struct ApproveJobResponse {
    pub success: bool,
}

/// Internal endpoint to approve a job for processing.
///
/// This marks a job as approved in the user's job record and also
/// updates the `approved` flag on the queue item in Firebase RTDB
/// so that workers can pick it up.
pub async fn approve_job(
    State(app): State<AppStatePtr>,
    Json(request): Json<ApproveJobRequest>,
) -> Result<Json<ApproveJobResponse>, AppError> {
    println!(
        "Received approval request for user {} job {}",
        request.user_id, request.job_index
    );

    // 1. Approve the job in the user's job record
    app.state
        .db
        .lock()
        .await
        .approve_job(&request.user_id, request.job_index)
        .await
        .context("Failed to approve job in database")?;

    // 2. Also update the queue item's `approved` flag in RTDB
    //    The queue item key is "{user_id}_{job_index}"
    let job_id = format!("{}_{}", request.user_id, request.job_index);
    let rtdb = FirebaseRtdb::from_env()
        .context("Failed to initialize Firebase RTDB client")?;

    // Check all stage queues for this job and approve it
    let stages = [
        StageNumber::Stage1MediaConversion,
        StageNumber::Stage2ValidityCheck,
        StageNumber::Stage3Reframing,
        StageNumber::Stage4PoseEstimation,
        StageNumber::Stage5CycleDetection,
        StageNumber::Stage6Prediction,
    ];

    let safe_job_id = job_id.replace('.', "_").replace('/', "_");

    for stage in stages {
        let path = format!("{}/{}", queue_path(stage), safe_job_id);
        let item: Option<QueueItem> = match rtdb.get(&path).await {
            Ok(item) => item,
            Err(_) => continue,
        };

        if let Some(mut item) = item {
            item.approved = true;
            if let Err(e) = rtdb.set(&path, &item).await {
                eprintln!("Warning: failed to approve queue item at {}: {}", path, e);
            } else {
                println!("Approved queue item at {}", path);
            }
            break; // Job can only be in one queue at a time
        }
    }

    Ok(Json(ApproveJobResponse { success: true }))
}
