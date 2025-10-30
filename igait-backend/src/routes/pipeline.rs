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

    // Get the prediction score
    let prediction_score = match output.result.clone() {
        Ok(score) => score,
        Err(e) => {
            tracing::error!("Pipeline failed for job {}: {}", job_id, e);
            tracing::error!("Pipeline stages output: {:#?}", output.stages);
            return Err(anyhow::anyhow!("Pipeline failed: {}", e));
        }
    };

    // Determine result type
    let result_type = if prediction_score > crate::ASD_CLASSIFICATION_THRESHOLD as f64 {
        "ASD"
    } else {
        "NO ASD"
    };

    tracing::info!("Job {} completed with score {} ({})", job_id, prediction_score, result_type);

    // Upload archive to S3 if present
    if let Some(archive_data) = archive_bytes {
        tracing::info!("Uploading results archive for job {} to S3", job_id);
        
        let s3_key = format!("results/{}/results.zip", job_id);
        
        let bucket = state.state.bucket.lock().await;
        bucket.put_object(&s3_key, &archive_data)
            .await
            .context("Failed to upload archive to S3")?;
        
        tracing::info!("Successfully uploaded archive to S3: {}", s3_key);
    }

    // Parse job_id which should be in format "uid_jobindex"
    // Example: "user123_5" means user "user123", job index 5
    let parts: Vec<&str> = job_id.split('_').collect();
    if parts.len() >= 2 {
        // Try to parse uid and job index
        if let Ok(job_index) = parts[parts.len() - 1].parse::<usize>() {
            let uid = parts[..parts.len() - 1].join("_");
            
            tracing::info!("Parsed job ID: uid={}, job_index={}", uid, job_index);
            
            // Update the database
            let db = state.state.db.lock().await;
            if let Err(e) = db.update_status(
                &uid,
                job_index,
                crate::helper::lib::JobStatus {
                    code: crate::helper::lib::JobStatusCode::Complete,
                    value: format!("{} (score: {:.2}%)", result_type, prediction_score * 100.0),
                },
            ).await {
                tracing::warn!("Failed to update database (this is expected if testing with fake user): {:?}", e);
                // Continue anyway since we have the results in S3
            } else {
                tracing::info!("Successfully updated database for job {}", job_id);
                
                // Send result email if not disabled
                if !crate::DISABLE_RESULT_EMAIL {
                    if let Ok(job) = db.get_job(&uid, job_index).await {
                        if let Err(e) = send_pipeline_result_email(
                            state.state.clone(),
                            &job.email,
                            result_type,
                            prediction_score,
                            &uid,
                            &job_id,
                        ).await {
                            tracing::error!("Failed to send result email: {:?}", e);
                        }
                    }
                }
            }
        } else {
            tracing::warn!("Could not parse job index from job_id: {}", job_id);
        }
    } else {
        tracing::warn!("Job ID format unexpected (should be uid_jobindex): {}", job_id);
    }

    Ok(())
}

/// Send a result email for pipeline completion
async fn send_pipeline_result_email(
    app: std::sync::Arc<crate::helper::lib::AppState>,
    recipient_email: &str,
    result_type: &str,
    score: f64,
    uid: &str,
    job_id: &str,
) -> Result<()> {
    let subject = "Your iGait Analysis Results".to_string();
    let body = format!(
        "Your gait analysis has been completed!<br><br>\
        Result: {}<br>\
        Confidence Score: {:.2}%<br><br>\
        Job ID: {}<br>\
        User ID: {}<br><br>\
        Thank you for using iGait!",
        result_type,
        score * 100.0,
        job_id,
        uid
    );
    
    crate::helper::email::send_email(app, recipient_email, &subject, &body).await
}
