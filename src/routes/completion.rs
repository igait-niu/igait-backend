use std::sync::Arc;

use axum::extract::{Multipart, State};
use chrono::{DateTime, Utc};
use tokio::sync::Mutex;
use anyhow::{ Result, Context, anyhow };

use crate::{helper::{email::{send_failure_email, send_success_email}, lib::{AppError, AppState, Job, JobStatus, JobStatusCode, JobTaskID}}, print_be};

/// The required arguments for the completion request.
struct CompletionRequestArguments {
    uid:              String,
    job_id:           usize,
    status_code:      String,
    status_content:   String,
    igait_access_key: String
}


/// Unpacks the completion request arguments from the multipart request into `CompletionRequestArguments`.
/// 
/// # Fails
/// * If any of the required fields are missing or the wrong data type
/// * If any of the fields are unreadable
/// * If the 'igait_access_key' is incorrect
/// 
/// # Arguments
/// * `multipart` - The multipart request to unpack
/// * `task_number` - The task number for logging
async fn unpack_completion_arguments(
    mut multipart: Multipart,
    task_number:   JobTaskID
) -> Result<CompletionRequestArguments> {
    let mut uid_option: Option<String> = None;
    let mut job_id_option: Option<usize> = None;
    let mut status_code_option: Option<String> = None;
    let mut status_content_option: Option<String> = None;
    let mut igait_access_key_option: Option<String> = None;

    while let Some(field) = multipart
        .next_field().await
        .context("Bad request! Is it possible you submitted a file over the size limit?")?
    {
        let name = field.name();
        print_be!(task_number, "Field Incoming: {name:#?}");
        match field.name() {
            Some("user_id") => {
                uid_option = Some(
                        field
                            .text().await
                            .context("Field 'user_id' wasn't readable as text!")?
                            .to_string());
            },
            Some("job_id") => {
                job_id_option = Some(
                        field
                            .text().await
                            .context("Field 'job_id' wasn't readable as text!")?
                            .to_string()
                            .parse::<usize>()
                            .context("Couldn't parse the incoming 'job_id' field!")?);
            },
            Some("status_code") => {
                status_code_option = Some(
                        field
                            .text().await
                            .context("Field 'status_code' wasn't readable as text!")?
                            .to_string());
            },
            Some("status_content") => {
                status_content_option = Some(
                        field
                            .text().await
                            .context("Field 'status_content' wasn't readable as text!")?
                            .to_string());
            },
            Some("igait_access_key") => {
                igait_access_key_option = Some(
                        field
                            .text().await
                            .context("Field 'igait_access_key' wasn't readable as text!")?
                            .to_string());
            },
            _ => {
                print_be!(task_number, "Which had an unknown/no field name...");
            }
        }
    }

    // Make sure all of the fields are present
    let uid = uid_option.ok_or(anyhow!("Missing 'user_id' in request!"))?;
    let job_id = job_id_option.ok_or(anyhow!("Missing 'job_id' in request!"))?;
    let status_code = status_code_option.ok_or(anyhow!("Missing 'status_code' in request!"))?;
    let status_content = status_content_option.ok_or(anyhow!("Missing 'status_content' in request!"))?;
    let igait_access_key = igait_access_key_option.ok_or(anyhow!("Missing 'igait_access_key' in request!"))?;

    Ok(CompletionRequestArguments {
        uid,
        job_id,
        status_code,
        status_content,
        igait_access_key
    })
}

/// The completion entrypoint for the server.
/// 
/// # Fails
/// * If the access key is incorrect
/// * If the job targeted by the completion request doesn't exist
/// * If the extensions file is unreadable
/// * If the extensions file is invalid UTF8
/// * If the extensions file is invalid JSON
/// * If the presigned URLs can't be generated
/// * If the success email can't be sent
/// * If the failure email can't be sent
/// 
/// # Arguments
/// * `State(app)` - The application state
/// * `multipart` - The multipart request
pub async fn completion_entrypoint (
    State(app): State<Arc<Mutex<AppState>>>,
    multipart: Multipart
) -> Result<&'static str, AppError> {
    // Allocate a new task number
    app.lock().await
        .task_number += 1;
    let task_number = app.lock().await.task_number;

    print_be!(task_number, "\n----- [ Recieved completion update ] -----");

    // Unpack the arguments from the request
    print_be!(task_number, "Unpacking arguments...");
    let arguments = unpack_completion_arguments(multipart, task_number).await
        .context("Failed to unpack completion arguments!")?;
    
    // First, check the access key against the environment
    if arguments.igait_access_key != std::env::var("IGAIT_ACCESS_KEY").context("MISSING 'IGAIT_ACCESS_KEY' in environment!")? {
        print_be!(task_number, "Invalid access key!");
        Err(anyhow!("Invalid access key from completion endpoint!"))?
    }

    // Build a new status object
    let mut status = JobStatus {
        code: JobStatusCode::Submitting,
        value: arguments.status_content.clone()
    };

    // Grab the job it references
    let job: Job = app.lock().await
        .db
        .get_job(
            &arguments.uid,
            arguments.job_id,
            task_number
        ).await
        .context("The job targeted by the completion request doesn't exist!")?; 

    // Extract the email address and timestamp
    let recipient_email_address = job.email.clone();
    let dt_timestamp_utc: DateTime<Utc> = job.timestamp.clone().into();

    if &arguments.status_code == "OK" {
        // This is a success, the inference process was completed
        print_be!(task_number, "Job successful!");
        status.code = JobStatusCode::Complete;

        // Update the status of the job
        app.lock().await
            .db.update_status(
                &arguments.uid,
                arguments.job_id,
                status.clone(),
                task_number
            ).await
            .context("Failed to update the status of the job! It was, however, successful.")?;

        // Extract the bytes from the extensions file
        let bytes: Vec<u8> = app.lock()
            .await
            .bucket
            .get_object(
                &format!("{}/{}/extensions.json",
                    arguments.uid,
                    arguments.job_id
                )
            ).await
            .context("Failed to get extensions file!")?
            .to_vec();

        // Convert the raw bytes to a string
        let extensions_as_string: String = String::from_utf8(bytes)
            .context("There was invalid UTF8 data from the `extensions.json` file!")?;

        // Parse the string into a JSON object
        let extensions: serde_json::Value = serde_json::from_str(&extensions_as_string)
            .context("Couldn't convert the `extensions.json` file to a JSON object!")?;

        // Generate the presigned URLs
        let front_keyframed_url = app.lock()
                .await
                .bucket
                .presign_get(
                    format!("{}/{}/front_keyframed.{}",
                        arguments.uid,
                        arguments.job_id,
                        extensions["front"]
                            .as_str()
                            .context("Invalid extension type for the front file!")?
                    ),
                    86400 * 7,
                    None
                )
                .context("Failed to get the front keyframed URL!")?;
        let side_keyframed_url = app.lock()
                .await
                .bucket
                .presign_get(
                    format!("{}/{}/side_keyframed.{}",
                        arguments.uid,
                        arguments.job_id,
                        extensions["side"]
                            .as_str()
                            .context("Invalid extension type for the side file!")?
                        ),
                    86400 * 7,
                    None
                )
                .context("Failed to get the side keyframed URL!")?;

        // Send the success email
        send_success_email(
            &recipient_email_address,
            &status,
            &dt_timestamp_utc,
            &job,
            &front_keyframed_url,
            &side_keyframed_url,
            &arguments.uid,
            arguments.job_id,
            task_number
        ).await.context("Failed to send success email!")?;
    } else if &arguments.status_code == "ERR" {
        // This is a failure, usually due to an error in the inference process
        let status_content = &arguments.status_content;
        print_be!(task_number, "Job unsuccessful - status content: '{status_content}'");
        status.code = JobStatusCode::InferenceErr;

        // Update the status of the job
        app.lock().await
            .db.update_status(
                &arguments.uid,
                arguments.job_id,
                status.clone(),
                task_number
            ).await
            .context("Failed to update the status of the job!")?;

        // Send the failure email
        send_failure_email(
            &recipient_email_address,
            &status,
            &dt_timestamp_utc,
            &arguments.uid,
            arguments.job_id,
            task_number
        ).await.context("Failed to send failure email!")?;
    } else {
        status.code = JobStatusCode::InferenceErr;

        // Update the status of the job
        app.lock().await
            .db.update_status(
                &arguments.uid,
                arguments.job_id,
                status.clone(),
                task_number
            ).await
            .context("Failed to update the status of the job!")?;

        // This is an invalid status code, probably a mistake or bad actor
        print_be!(task_number, "Invalid status code!");
        return Err(AppError(anyhow!("Invalid status code from completion endpoint!")));
    }
    
    Ok("OK")
}