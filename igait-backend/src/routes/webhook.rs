//! Stage webhook endpoints for receiving callbacks from microservices.
//!
//! This module handles callbacks from stage microservices when they complete
//! processing a job (success or failure).

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use anyhow::{Context, Result};

use igait_lib::microservice::{StageJobResult, StageJobRequest, StageNumber, StageResultStatus, JobMetadata};

use crate::helper::lib::{AppStatePtr, JobStatus, JobStatusCode};

/// Webhook endpoint that receives stage completion callbacks.
///
/// # Path Parameters
/// * `stage_num` - The stage number that completed (1-7)
///
/// # Body
/// * `StageJobResult` - The result from the completed stage
///
/// # Returns
/// * `200 OK` if the result was processed successfully
/// * `400 Bad Request` if the stage number is invalid
/// * `500 Internal Server Error` if processing fails
pub async fn stage_webhook_entrypoint(
    Path(stage_num): Path<u8>,
    State(state): State<AppStatePtr>,
    Json(result): Json<StageJobResult>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    println!(
        "Received webhook for stage {} - job {} - status {:?}",
        stage_num, result.job_id, result.status
    );

    // Validate stage number
    if !(1..=7).contains(&stage_num) {
        return Err((
            StatusCode::BAD_REQUEST,
            format!("Invalid stage number: {}. Must be 1-7.", stage_num),
        ));
    }

    // Process the stage result
    if let Err(e) = process_stage_result(&state, stage_num, result).await {
        eprintln!("Failed to process stage {} result: {:?}", stage_num, e);
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to process stage result: {}", e),
        ));
    }

    Ok((StatusCode::OK, "Stage result processed successfully"))
}

/// Process a stage completion result.
///
/// On success: Update database, dispatch to next stage (or finalize if stage 7)
/// On failure: Update database with error status, send failure email
async fn process_stage_result(
    state: &AppStatePtr,
    stage_num: u8,
    result: StageJobResult,
) -> Result<()> {
    // Parse job_id (format: "uid_jobindex")
    let (uid, job_index) = parse_job_id(&result.job_id)?;

    match result.status {
        StageResultStatus::Success => {
            println!(
                "Stage {} succeeded for job {} (took {}ms)",
                stage_num, result.job_id, result.duration_ms
            );

            // Update status in database
            let status = JobStatus {
                code: JobStatusCode::Processing,
                value: format!("Stage {} complete. Processing...", stage_num),
            };

            state
                .state
                .db
                .lock()
                .await
                .update_status(&uid, job_index, status)
                .await
                .context("Failed to update job status")?;

            // Dispatch to next stage or finalize
            if stage_num < 7 {
                dispatch_next_stage(state, stage_num + 1, &result).await?;
            } else {
                finalize_job(state, &result).await?;
            }
        }
        StageResultStatus::Failed => {
            let error_msg = result.error.as_deref().unwrap_or("Unknown error");
            eprintln!(
                "Stage {} failed for job {}: {}",
                stage_num, result.job_id, error_msg
            );

            // Update status in database
            let status = JobStatus {
                code: JobStatusCode::InferenceErr,
                value: format!("Stage {} failed: {}", stage_num, error_msg),
            };

            state
                .state
                .db
                .lock()
                .await
                .update_status(&uid, job_index, status)
                .await
                .context("Failed to update job status")?;

            // Send failure email
            send_stage_failure_email(state, &uid, job_index, stage_num, error_msg).await?;
        }
        StageResultStatus::Skipped => {
            println!("Stage {} was skipped for job {}", stage_num, result.job_id);

            // Continue to next stage
            if stage_num < 7 {
                dispatch_next_stage(state, stage_num + 1, &result).await?;
            } else {
                finalize_job(state, &result).await?;
            }
        }
    }

    Ok(())
}

/// Parse a job ID into (uid, job_index).
fn parse_job_id(job_id: &str) -> Result<(String, usize)> {
    let parts: Vec<&str> = job_id.rsplitn(2, '_').collect();
    if parts.len() != 2 {
        anyhow::bail!("Invalid job_id format: {}. Expected 'uid_index'.", job_id);
    }

    let job_index: usize = parts[0]
        .parse()
        .context(format!("Failed to parse job index from: {}", parts[0]))?;
    let uid = parts[1].to_string();

    Ok((uid, job_index))
}

/// Dispatch a job to the next stage service.
async fn dispatch_next_stage(
    _state: &AppStatePtr,
    next_stage: u8,
    prev_result: &StageJobResult,
) -> Result<()> {
    let stage = StageNumber::from_u8(next_stage)
        .context(format!("Invalid stage number: {}", next_stage))?;

    let (uid, _) = parse_job_id(&prev_result.job_id)?;

    // Build callback URL for this stage
    let callback_url = std::env::var("BACKEND_CALLBACK_URL")
        .map(|base_url| format!("{}/{}", base_url, next_stage))
        .unwrap_or_else(|_| format!("http://localhost:3000/api/v1/webhook/stage/{}", next_stage));

    // The output_keys from the previous stage become input_keys for the next stage
    let request = StageJobRequest {
        job_id: prev_result.job_id.clone(),
        user_id: uid,
        stage,
        callback_url,
        input_keys: prev_result.output_keys.clone(),
        metadata: JobMetadata::default(),
    };

    // Get the service URL for this stage
    let service_url = get_stage_service_url(next_stage);

    println!(
        "Dispatching job {} to stage {} at {}",
        prev_result.job_id, next_stage, service_url
    );

    let client = reqwest::Client::new();
    let response = client
        .post(&format!("{}/submit", service_url))
        .json(&request)
        .send()
        .await
        .context(format!("Failed to connect to stage {} service", next_stage))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        anyhow::bail!(
            "Stage {} service returned error {}: {}",
            next_stage,
            status,
            body
        );
    }

    println!(
        "Successfully dispatched job {} to stage {}",
        prev_result.job_id, next_stage
    );
    Ok(())
}

/// Get the service URL for a given stage.
fn get_stage_service_url(stage: u8) -> String {
    let env_var = format!("STAGE{}_SERVICE_URL", stage);
    let default = format!("http://localhost:800{}/", stage);
    std::env::var(&env_var).unwrap_or(default)
}

/// Finalize a completed job (stage 7 done).
async fn finalize_job(state: &AppStatePtr, result: &StageJobResult) -> Result<()> {
    let (uid, job_index) = parse_job_id(&result.job_id)?;

    println!("Finalizing job {} - pipeline complete!", result.job_id);

    // Get the prediction result from stage 7 output
    // The score should be in the output_keys under "prediction_score"
    let score: f64 = result
        .output_keys
        .get("prediction_score")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0.0);

    let result_type = if score > crate::ASD_CLASSIFICATION_THRESHOLD as f64 {
        "ASD"
    } else {
        "NO ASD"
    };

    // Update status in database
    let status = JobStatus {
        code: JobStatusCode::Complete,
        value: format!("{} (score: {:.2}%)", result_type, score * 100.0),
    };

    state
        .state
        .db
        .lock()
        .await
        .update_status(&uid, job_index, status)
        .await
        .context("Failed to update job status")?;

    // Send success email
    send_success_email(state, &uid, job_index, &result.job_id, score).await?;

    println!("Job {} finalized successfully!", result.job_id);
    Ok(())
}

/// Send a success email for completed analysis.
async fn send_success_email(
    state: &AppStatePtr,
    uid: &str,
    job_index: usize,
    job_id: &str,
    _score: f64,
) -> Result<()> {
    let db = state.state.db.lock().await;
    let job = db
        .get_job(uid, job_index)
        .await
        .context("Failed to get job for email")?;

    let subject = "Your iGait Analysis is Complete";
    let body = format!(
        "Dear iGAIT user,<br><br>\
        Your gait analysis has been completed successfully!<br><br>\
        Please understand that the iGAIT website is still under development. \
        The research team will review your screening result and may contact you if needed.<br><br>\
        Job ID: {}<br><br>\
        If you have any questions, please contact us at GaitStudy@niu.edu.<br><br>\
        Thank you for using iGait!",
        job_id
    );

    drop(db); // Release lock before async email send

    crate::helper::email::send_email(state.state.clone(), &job.email, subject, &body).await
}

/// Send a failure email when a stage fails.
async fn send_stage_failure_email(
    state: &AppStatePtr,
    uid: &str,
    job_index: usize,
    stage_num: u8,
    error_msg: &str,
) -> Result<()> {
    let db = state.state.db.lock().await;
    let job = match db.get_job(uid, job_index).await {
        Ok(j) => j,
        Err(e) => {
            eprintln!("Could not get job for failure email: {:?}", e);
            return Ok(());
        }
    };

    let (subject, body) = if stage_num == 2 && error_msg.contains("person") {
        // Special case for validity check failure
        (
            "Unable to Process Your iGait Submission".to_string(),
            format!(
                "Dear iGAIT user,<br><br>\
                We were unable to process your gait analysis submission because a person could not be detected in the uploaded videos.<br><br>\
                Please ensure that:<br>\
                <ul>\
                <li>The videos clearly show a person walking</li>\
                <li>The person is visible and not obscured</li>\
                <li>The lighting is adequate</li>\
                <li>The camera is stable and properly positioned</li>\
                </ul><br>\
                Please try recording and submitting new videos following our guidelines.<br><br>\
                Thank you for your understanding."
            ),
        )
    } else {
        (
            "Your iGait Analysis Encountered an Error".to_string(),
            format!(
                "Dear iGAIT user,<br><br>\
                Unfortunately, there was an error processing your gait analysis submission during stage {}.<br><br>\
                Error: {}<br><br>\
                Please try submitting again. If the problem persists, contact us at GaitStudy@niu.edu for assistance.<br><br>\
                We apologize for the inconvenience.",
                stage_num, error_msg
            ),
        )
    };

    drop(db); // Release lock before async email send

    crate::helper::email::send_email(state.state.clone(), &job.email, &subject, &body).await
}
