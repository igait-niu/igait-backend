//! This module contains the pipeline submission endpoint for the API.
//! 
//! This endpoint receives results from the iGait pipeline running on the compute cluster.
//! It is secured with a shared secret to ensure only authorized pipeline instances can submit.

use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum::http::HeaderMap;
use igait_lib::Output;
use crate::helper::lib::AppStatePtr;
use anyhow::{Context, Result};

/// Pipeline submission endpoint that receives results from the compute cluster
///
/// # Arguments
/// * `headers`: HTTP headers containing the X-Pipeline-Secret
/// * `state`: Application state
/// * `multipart`: Multipart form data containing:
///   - `output`: JSON-serialized Output struct
///   - `archive`: Optional results.zip file
///
/// # Returns
/// * `200 OK` if successful
/// * `401 Unauthorized` if secret is invalid
/// * `400 Bad Request` if data is malformed
/// * `500 Internal Server Error` if processing fails
#[tracing::instrument(skip(headers, state, multipart))]
pub async fn pipeline_submit_entrypoint(
    headers: HeaderMap,
    State(state): State<AppStatePtr>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // Verify the secret
    let secret = headers
        .get("X-Pipeline-Secret")
        .and_then(|v| v.to_str().ok())
        .ok_or((StatusCode::UNAUTHORIZED, "Missing or invalid secret".to_string()))?;
    
    let expected_secret = std::env::var("PIPELINE_SECRET")
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Server misconfigured".to_string()))?;
    
    if secret != expected_secret {
        return Err((StatusCode::UNAUTHORIZED, "Invalid secret".to_string()));
    }

    let mut output: Option<Output> = None;
    let mut archive_bytes: Option<Vec<u8>> = None;

    // Parse multipart data
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Failed to read multipart field: {}", e)))?
    {
        let name = field.name().unwrap_or("").to_string();
        
        match name.as_str() {
            "output" => {
                let data = field.bytes().await
                    .map_err(|e| (StatusCode::BAD_REQUEST, format!("Failed to read output: {}", e)))?;
                output = Some(serde_json::from_slice(&data)
                    .map_err(|e| (StatusCode::BAD_REQUEST, format!("Failed to parse output JSON: {}", e)))?);
            }
            "archive" => {
                archive_bytes = Some(field.bytes().await
                    .map_err(|e| (StatusCode::BAD_REQUEST, format!("Failed to read archive: {}", e)))?
                    .to_vec());
            }
            _ => {}
        }
    }

    let output = output.ok_or((StatusCode::BAD_REQUEST, "Missing output field".to_string()))?;

    // Process the submission
    if let Err(e) = process_pipeline_submission(&state, output, archive_bytes).await {
        tracing::error!("Failed to process pipeline submission: {:?}", e);
        return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to process submission: {}", e)));
    }

    Ok((StatusCode::OK, "Pipeline submission processed successfully"))
}

/// Process the pipeline submission by updating the database and storing results
async fn process_pipeline_submission(
    state: &AppStatePtr,
    output: Output,
    archive_bytes: Option<Vec<u8>>,
) -> Result<()> {
    // Extract job ID from output directory path
    let job_id = output.canonical_paths.output_dir
        .file_name()
        .and_then(|n| n.to_str())
        .context("Failed to extract job ID from output path")?
        .to_string();
    
    tracing::info!("Processing pipeline submission for job ID: {}", job_id);
    
    // Parse job_id which should be in format "uid_jobindex"
    let parts: Vec<&str> = job_id.split('_').collect();
    let (uid, job_index) = if parts.len() >= 2 {
        if let Ok(job_index) = parts[parts.len() - 1].parse::<usize>() {
            let uid = parts[..parts.len() - 1].join("_");
            (uid, job_index)
        } else {
            tracing::warn!("Could not parse job index from job_id: {}", job_id);
            return Ok(()); // Don't fail the request, just log and return
        }
    } else {
        tracing::warn!("Job ID format unexpected (should be uid_jobindex): {}", job_id);
        return Ok(()); // Don't fail the request, just log and return
    };
    
    tracing::info!("Parsed job ID: uid={}, job_index={}", uid, job_index);
    
    // Check if the pipeline succeeded or failed
    match output.result.clone() {
        Ok(score) => {
            // Pipeline succeeded - handle success case
            let result_type = if score > crate::ASD_CLASSIFICATION_THRESHOLD as f64 {
                "ASD"
            } else {
                "NO ASD"
            };
            
            tracing::info!("Job {} completed successfully with score {} ({})", job_id, score, result_type);
            
            // Upload archive to S3 if present
            if let Some(archive_data) = archive_bytes {
                tracing::info!("Uploading results archive for job {} to S3", job_id);
                
                let s3_key = format!("results/{}/results.zip", job_id);
                
                let bucket = state.state.bucket.lock().await;
                if let Err(e) = bucket.put_object(&s3_key, &archive_data).await {
                    tracing::error!("Failed to upload archive to S3: {:?}", e);
                    // Continue anyway - we can still update the database
                } else {
                    tracing::info!("Successfully uploaded archive to S3: {}", s3_key);
                }
            }
            
            // Update the database with success
            let db = state.state.db.lock().await;
            if let Err(e) = db.update_status(
                &uid,
                job_index,
                crate::helper::lib::JobStatus {
                    code: crate::helper::lib::JobStatusCode::Complete,
                    value: format!("{} (score: {:.2}%)", result_type, score * 100.0),
                },
            ).await {
                tracing::warn!("Failed to update database (this is expected if testing with fake user): {:?}", e);
            } else {
                tracing::info!("Successfully updated database for job {}", job_id);
                
                // Always send success email (opaque if DISABLE_RESULT_EMAIL is true)
                if let Ok(job) = db.get_job(&uid, job_index).await {
                    if let Err(e) = send_success_email(
                        state.state.clone(),
                        &job.email,
                        &uid,
                        &job_id,
                    ).await {
                        tracing::error!("Failed to send success email: {:?}", e);
                    }
                }
            }
        }
        Err(e) => {
            // Pipeline failed - handle failure case
            tracing::error!("Pipeline failed for job {}: {}", job_id, e);
            tracing::error!("Pipeline stages output: {:#?}", output.stages);
            
            // Determine if this is a stage 2 failure or generic failure
            let is_stage2_failure = e.contains("Stage 2") || e.contains("Validity Check");
            
            // Update the database with failure status
            let db = state.state.db.lock().await;
            let failure_status = crate::helper::lib::JobStatus {
                code: crate::helper::lib::JobStatusCode::InferenceErr,
                value: if is_stage2_failure {
                    "Unable to detect person in video".to_string()
                } else {
                    format!("Processing failed: {}", e.lines().next().unwrap_or(&e))
                },
            };
            
            if let Err(e) = db.update_status(&uid, job_index, failure_status).await {
                tracing::warn!("Failed to update database with error status: {:?}", e);
            } else {
                tracing::info!("Updated database with failure status for job {}", job_id);
            }
            
            // Send appropriate failure email
            if let Ok(job) = db.get_job(&uid, job_index).await {
                let email_result = if is_stage2_failure {
                    send_stage2_failure_email(
                        state.state.clone(),
                        &job.email,
                        &uid,
                        &job_id,
                    ).await
                } else {
                    send_generic_failure_email(
                        state.state.clone(),
                        &job.email,
                        &uid,
                        &job_id,
                        &e,
                    ).await
                };
                
                if let Err(e) = email_result {
                    tracing::error!("Failed to send failure email: {:?}", e);
                }
            }
            
            // Upload partial results if archive is present (for debugging)
            if let Some(archive_data) = archive_bytes {
                tracing::info!("Uploading partial results archive for failed job {} to S3", job_id);
                
                let s3_key = format!("results/{}/partial_results.zip", job_id);
                
                let bucket = state.state.bucket.lock().await;
                if let Err(e) = bucket.put_object(&s3_key, &archive_data).await {
                    tracing::error!("Failed to upload partial archive to S3: {:?}", e);
                } else {
                    tracing::info!("Successfully uploaded partial archive to S3: {}", s3_key);
                }
            }
        }
    }
    
    Ok(())
}

/// Send a success email for pipeline completion (censored when DISABLE_RESULT_EMAIL is true)
async fn send_success_email(
    app: std::sync::Arc<crate::helper::lib::AppState>,
    recipient_email: &str,
    uid: &str,
    job_id: &str,
) -> Result<()> {
    let subject = "Your iGait Analysis is Complete".to_string();
    let body = if crate::DISABLE_RESULT_EMAIL {
        format!(
            "Dear iGAIT user,<br><br>\
            Your gait analysis has been completed successfully!<br><br>\
            Please understand that the iGAIT website is still under development. \
            The research team will review your screening result and may contact you if needed.<br><br>\
            Job ID: {}<br>\
            User ID: {}<br><br>\
            If you have any questions, please contact us at GaitStudy@niu.edu.<br><br>\
            Thank you for using iGait!",
            job_id,
            uid
        )
    } else {
        format!(
            "Dear iGAIT user,<br><br>\
            Your gait analysis has been completed successfully!<br><br>\
            Job ID: {}<br>\
            User ID: {}<br><br>\
            Thank you for using iGait!",
            job_id,
            uid
        )
    };
    
    crate::helper::email::send_email(app, recipient_email, &subject, &body).await
}

/// Send a generic failure email
async fn send_generic_failure_email(
    app: std::sync::Arc<crate::helper::lib::AppState>,
    recipient_email: &str,
    uid: &str,
    job_id: &str,
    error_message: &str,
) -> Result<()> {
    let subject = "Your iGait Analysis Encountered an Error".to_string();
    let body = format!(
        "Dear iGAIT user,<br><br>\
        Unfortunately, there was an error processing your gait analysis submission.<br><br>\
        Job ID: {}<br>\
        User ID: {}<br><br>\
        Error details: {}<br><br>\
        Please try submitting again. If the problem persists, contact us at GaitStudy@niu.edu for assistance.<br><br>\
        We apologize for the inconvenience.",
        job_id,
        uid,
        error_message
    );
    
    crate::helper::email::send_email(app, recipient_email, &subject, &body).await
}

/// Send a stage 2 failure email (person could not be detected)
async fn send_stage2_failure_email(
    app: std::sync::Arc<crate::helper::lib::AppState>,
    recipient_email: &str,
    uid: &str,
    job_id: &str,
) -> Result<()> {
    let subject = "Unable to Process Your iGait Submission".to_string();
    let body = format!(
        "Dear iGAIT user,<br><br>\
        We were unable to process your gait analysis submission because a person could not be detected in the uploaded videos.<br><br>\
        Job ID: {}<br>\
        User ID: {}<br><br>\
        Please ensure that:<br>\
        <ul>\
        <li>The videos clearly show a person walking</li>\
        <li>The person is visible and not obscured</li>\
        <li>The lighting is adequate</li>\
        <li>The camera is stable and properly positioned</li>\
        </ul><br>\
        Please try recording and submitting new videos following our guidelines. \
        If you continue to experience issues, contact us at GaitStudy@niu.edu for assistance.<br><br>\
        Thank you for your understanding.",
        job_id,
        uid
    );
    
    crate::helper::email::send_email(app, recipient_email, &subject, &body).await
}
