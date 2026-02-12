//! Rerun endpoint for re-processing a job from a specific stage.
//!
//! This endpoint allows administrators to rerun any job starting from
//! a given stage. It cleans up S3 outputs from the target stage onward
//! and re-inserts the job into the target stage's queue.
//!
//! Only users with `administrator: true` in the database are authorised.

use std::collections::HashMap;

use axum::{extract::State, Json};
use anyhow::{Context, anyhow};
use firebase_auth::FirebaseUser;
use serde::{Deserialize, Serialize};

use igait_lib::microservice::{
    JobMetadata, QueueItem, StageNumber, StoragePaths,
    FirebaseRtdb, queue_item_path,
};

use crate::helper::lib::{AppError, AppStatePtr, JobStatus, NUM_STAGES};

/// Request body for the rerun endpoint.
#[derive(Debug, Deserialize)]
pub struct RerunRequest {
    /// The UID of the user who owns the job.
    /// Admins must specify this to indicate whose job to rerun.
    pub user_id: String,
    /// The index of the job in the user's job list (0-indexed).
    pub job_index: usize,
    /// The stage number to restart from (1–7).
    pub stage: u8,
}

/// Response body for the rerun endpoint.
#[derive(Debug, Serialize)]
pub struct RerunResponse {
    /// Whether the rerun was successfully initiated.
    pub success: bool,
    /// Human-readable message.
    pub message: String,
    /// Number of S3 objects deleted during cleanup.
    pub objects_deleted: usize,
}

/// Authenticated endpoint to rerun a job from a specific stage.
/// **Admin-only** — the caller must have `administrator: true`.
///
/// # Workflow
/// 1. Verify the caller is an administrator
/// 2. Validate the stage number
/// 3. Fetch the target user's job
/// 4. Delete S3 outputs for stages `stage..=7`
/// 5. Reconstruct a `QueueItem` with the correct input keys
/// 6. Push the item into the target stage's queue in Firebase RTDB
/// 7. Update the job status to "Processing" for the target stage
///
/// # Arguments
/// * `current_user` – The Firebase-authenticated user (extracted from Bearer token).
/// * `app` – The shared application state.
/// * `request` – JSON body with `user_id`, `job_index`, and `stage`.
pub async fn rerun_entrypoint(
    current_user: FirebaseUser,
    State(app): State<AppStatePtr>,
    Json(request): Json<RerunRequest>,
) -> Result<Json<RerunResponse>, AppError> {
    let app = app.state;
    let caller_uid = &current_user.user_id;
    let target_uid = &request.user_id;
    let stage = request.stage;
    let job_index = request.job_index;

    // ── 0. Verify the caller is an administrator ────────────────────
    let caller = app
        .db
        .lock()
        .await
        .get_user(caller_uid)
        .await
        .context("Failed to look up caller in the database")?;

    if !caller.administrator {
        return Err(AppError(anyhow!(
            "Forbidden: only administrators may rerun jobs."
        )));
    }

    // ── 1. Validate stage number ────────────────────────────────────
    if stage < 1 || stage > NUM_STAGES {
        return Err(AppError(anyhow!(
            "Invalid stage number {}. Must be between 1 and {}.",
            stage,
            NUM_STAGES
        )));
    }

    let target_stage = StageNumber::from_u8(stage)
        .ok_or_else(|| anyhow!("Failed to convert stage number {} to StageNumber", stage))?;

    // ── 2. Fetch the job ────────────────────────────────────────────
    let job = app
        .db
        .lock()
        .await
        .get_job(target_uid, job_index)
        .await
        .context("Failed to fetch the job — does it exist?")?;

    let job_id = format!("{}_{}", target_uid, job_index);
    println!("Rerun requested by admin {}: job={}, stage={}", caller_uid, job_id, stage);

    // ── 3. Delete S3 outputs for stages `stage..=7` ─────────────────
    let mut total_deleted: usize = 0;
    for s in stage..=NUM_STAGES {
        let prefix = StoragePaths::stage_dir(&job_id, s);
        let deleted = app
            .storage
            .delete_by_prefix(&prefix)
            .await
            .context(format!("Failed to delete S3 objects for stage {}", s))?;
        println!("Deleted {} object(s) from {}", deleted, prefix);
        total_deleted += deleted;
    }

    // ── 4. Build input keys for the target stage ────────────────────
    // The target stage reads from the *previous* stage's output directory.
    // For stage 1 the inputs are the original uploads (stage_0).
    let input_keys = build_input_keys(&job_id, stage);

    // Build metadata from the job record
    let metadata = JobMetadata {
        email: Some(job.email.clone()),
        age: Some(job.age),
        sex: Some(job.sex.to_string().chars().next().unwrap_or('O')),
        ethnicity: Some(job.ethnicity.to_string()),
        height: Some(job.height.clone()),
        weight: Some(job.weight),
        extra: HashMap::new(),
    };

    let queue_item = QueueItem::new(
        job_id.clone(),
        target_uid.to_string(),
        input_keys,
        metadata,
        job.requires_approval,
    );

    // ── 5. Push into the target stage's queue ───────────────────────
    let rtdb = FirebaseRtdb::from_env()
        .context("Failed to initialise Firebase RTDB client")?;

    let path = queue_item_path(target_stage, &job_id);
    rtdb.set(&path, &queue_item)
        .await
        .context("Failed to push job to the target stage queue")?;

    println!("Job {} pushed to stage {} queue", job_id, stage);

    // ── 6. Update job status ────────────────────────────────────────
    let status = JobStatus::processing(stage);
    app.db
        .lock()
        .await
        .update_status(target_uid, job_index, status)
        .await
        .context("Failed to update job status")?;

    Ok(Json(RerunResponse {
        success: true,
        message: format!(
            "Job {} is being re-processed from stage {} ({}).",
            job_id,
            stage,
            target_stage.name()
        ),
        objects_deleted: total_deleted,
    }))
}

/// Builds the `input_keys` map for the target stage.
///
/// Each stage expects `front_video` and `side_video` keys pointing
/// at the previous stage's outputs. For stage 1, inputs come from
/// the original uploads in `stage_0`.
fn build_input_keys(job_id: &str, stage: u8) -> HashMap<String, String> {
    let prev = stage.saturating_sub(1);
    let mut keys = HashMap::new();

    // Convention: previous stage outputs are `front.mp4` / `side.mp4`
    keys.insert(
        "front_video".to_string(),
        StoragePaths::stage_front_video(job_id, prev, "mp4"),
    );
    keys.insert(
        "side_video".to_string(),
        StoragePaths::stage_side_video(job_id, prev, "mp4"),
    );

    keys
}
