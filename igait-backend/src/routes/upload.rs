//! Upload endpoint for submitting new gait analysis jobs.
//!
//! This module handles video uploads and initiates the processing pipeline
//! by uploading to Firebase Storage and dispatching to Stage 1.

use std::{collections::HashMap, sync::Arc, time::SystemTime};

use axum::{body::Bytes, extract::{Multipart, State}};
use anyhow::{Result, Context, anyhow};

use igait_lib::microservice::{StoragePaths, StageJobRequest, StageNumber, JobMetadata};

use crate::helper::{
    email::send_welcome_email,
    lib::{AppError, AppState, AppStatePtr, Job, JobStatus, JobStatusCode},
};

/// The required arguments for the upload request.
struct UploadRequestArguments {
    uid:        String,
    age:        i16,
    ethnicity:  String,
    sex:        char,
    height:     String,
    weight:     i16,
    email:      String,
    front_file: UploadRequestFile,
    side_file:  UploadRequestFile,
}

/// A representation of a file in a `Multipart` request.
#[derive(Debug)]
struct UploadRequestFile {
    name:  String,
    bytes: Bytes,
}

/// Takes in the `Multipart` request and unpacks the arguments into an `UploadRequestArguments` object.
///
/// # Fails
/// If any of the fields are missing or if the files are too large.
///
/// # Arguments
/// * `multipart` - The `Multipart` object to unpack.
async fn unpack_upload_arguments(multipart: &mut Multipart) -> Result<UploadRequestArguments> {
    // Initialize all of the fields as options
    let mut uid_option:       Option<String> = None;
    let mut age_option:       Option<i16>    = None;
    let mut ethnicity_option: Option<String> = None;
    let mut sex_option:       Option<char>   = None;
    let mut height_option:    Option<String> = None;
    let mut weight_option:    Option<i16>    = None;
    let mut email_option:     Option<String> = None;

    // Initialize the file fields as options
    let mut front_file_name_option:  Option<String> = None;
    let mut side_file_name_option:   Option<String> = None;
    let mut front_file_bytes_option: Option<Bytes>  = None;
    let mut side_file_bytes_option:  Option<Bytes>  = None;

    // Loop through the fields
    while let Some(field) = multipart
        .next_field()
        .await
        .context("Bad upload request! Is it possible you submitted a file over the size limit?")?
    {
        let name = field.name();
        let field_name = field.file_name();
        println!("Field Incoming: {name:?} - File Attached: {field_name:?}");

        match field.name() {
            Some("fileuploadfront") => {
                front_file_name_option = field.file_name().map(String::from);
                front_file_bytes_option = Some(
                    field
                        .bytes()
                        .await
                        .context("Could not unpack bytes from field 'fileuploadfront'!")?,
                );
            }
            Some("fileuploadside") => {
                side_file_name_option = field.file_name().map(String::from);
                side_file_bytes_option = Some(
                    field
                        .bytes()
                        .await
                        .context("Could not unpack bytes from field 'fileuploadside'!")?,
                );
            }
            Some("uid") => {
                uid_option = Some(
                    field
                        .text()
                        .await
                        .context("Field 'uid' wasn't readable as text!")?,
                );
            }
            Some("age") => {
                age_option = Some(
                    field
                        .text()
                        .await
                        .context("Field 'age' wasn't readable as text!")?
                        .parse()
                        .context("Field 'age' wasn't parseable as a number!")?,
                );
            }
            Some("ethnicity") => {
                ethnicity_option = Some(
                    field
                        .text()
                        .await
                        .context("Field 'ethnicity' wasn't readable as text!")?,
                );
            }
            Some("email") => {
                email_option = Some(
                    field
                        .text()
                        .await
                        .context("Field 'email' wasn't readable as text!")?,
                );
            }
            Some("sex") => {
                sex_option = Some(
                    field
                        .text()
                        .await
                        .context("Field 'sex' wasn't readable as text!")?
                        .chars()
                        .next()
                        .context("Field 'sex' was empty!")?,
                );
            }
            Some("height") => {
                height_option = Some(
                    field
                        .text()
                        .await
                        .context("Field 'height' wasn't readable as text!")?,
                );
            }
            Some("weight") => {
                weight_option = Some(
                    field
                        .text()
                        .await
                        .context("Field 'weight' wasn't readable as text!")?
                        .parse()
                        .context("Field 'weight' wasn't parseable as a number!")?,
                );
            }
            _ => {
                println!("Skipping unknown field: {name:?}");
            }
        }
    }

    // Make sure all of the fields are present
    let uid       = uid_option.ok_or(anyhow!("Missing 'uid' in request!"))?;
    let age       = age_option.ok_or(anyhow!("Missing 'age' in request"))?;
    let ethnicity = ethnicity_option.ok_or(anyhow!("Missing 'ethnicity' in request"))?;
    let sex       = sex_option.ok_or(anyhow!("Missing 'sex' in request"))?;
    let height    = height_option.ok_or(anyhow!("Missing 'height' in request"))?;
    let weight    = weight_option.ok_or(anyhow!("Missing 'weight' in request"))?;
    let email     = email_option.ok_or(anyhow!("Missing 'email' in request"))?;

    // Make sure all of the file fields are present
    let front_file_name  = front_file_name_option.ok_or(anyhow!("Missing 'fileuploadfront' in request!"))?;
    let side_file_name   = side_file_name_option.ok_or(anyhow!("Missing 'fileuploadside' in request!"))?;
    let front_file_bytes = front_file_bytes_option.ok_or(anyhow!("Missing 'fileuploadfront' bytes!"))?;
    let side_file_bytes  = side_file_bytes_option.ok_or(anyhow!("Missing 'fileuploadside' bytes!"))?;

    Ok(UploadRequestArguments {
        uid,
        age,
        ethnicity,
        sex,
        height,
        weight,
        email,
        front_file: UploadRequestFile {
            name: front_file_name,
            bytes: front_file_bytes,
        },
        side_file: UploadRequestFile {
            name: side_file_name,
            bytes: side_file_bytes,
        },
    })
}

/// The entrypoint for the upload request.
///
/// # Workflow
/// 1. Parse and validate the multipart form data
/// 2. Create a new job in the database
/// 3. Upload videos to Firebase Storage
/// 4. Dispatch to Stage 1 microservice
/// 5. Send welcome email
///
/// # Fails
/// * If the arguments are missing or invalid
/// * If the files fail to upload to Firebase Storage
/// * If the job fails to save to the database
/// * If the welcome email fails to send
pub async fn upload_entrypoint(
    State(app): State<AppStatePtr>,
    mut multipart: Multipart,
) -> Result<(), AppError> {
    let app = app.state;

    println!("Unpacking upload arguments...");
    let arguments = unpack_upload_arguments(&mut multipart)
        .await
        .context("Failed to unpack arguments!")?;

    // Build a new status object
    let mut status = JobStatus {
        code: JobStatusCode::Submitting,
        value: String::from("Uploading files..."),
    };

    // Generate the new job ID (0-indexed)
    let job_index = app
        .db
        .lock()
        .await
        .count_jobs(&arguments.uid)
        .await
        .context("Failed to count the number of jobs!")?;

    // Build job ID string (format: "{user_id}_{job_index}")
    let job_id = format!("{}_{}", arguments.uid, job_index);
    println!("Created job ID: {}", job_id);

    // Build the new job object
    let job = Job {
        age:       arguments.age,
        ethnicity: arguments.ethnicity.clone(),
        sex:       arguments.sex,
        height:    arguments.height.clone(),
        weight:    arguments.weight,
        status:    status.clone(),
        email:     arguments.email.clone(),
        timestamp: SystemTime::now(),
    };

    // Add the job to the database
    app.db
        .lock()
        .await
        .new_job(&arguments.uid, job.clone())
        .await
        .context("Failed to add the new job to the database!")?;

    // Extract values we need before moving files
    let age = arguments.age;
    let sex = arguments.sex;

    // Upload files to Firebase Storage and dispatch to Stage 1
    if let Err(err) = upload_and_dispatch(
        app.clone(),
        &job_id,
        &arguments.uid,
        arguments.front_file,
        arguments.side_file,
        age,
        sex,
    )
    .await
    {
        // Populate the status object with error
        status.code = JobStatusCode::SubmissionErr;
        status.value = err.to_string();

        // Update the status of the job
        app.db
            .lock()
            .await
            .update_status(&arguments.uid, job_index, status)
            .await
            .context("Failed to update the status of the job!")?;

        return Err(AppError(err.context("Failed to upload files or dispatch job!")));
    }

    // Update status to Queue
    status.code = JobStatusCode::Queue;
    status.value = String::from("Job submitted for processing.");

    // Send the welcome email
    send_welcome_email(app.clone(), &job, &arguments.uid, job_index)
        .await
        .context("Failed to send welcome email!")?;

    // Update the status of the job
    app.db
        .lock()
        .await
        .update_status(&arguments.uid, job_index, status)
        .await
        .context("Failed to update the status of the job!")?;

    println!("Job {} submitted successfully!", job_id);
    Ok(())
}

/// Uploads files to Firebase Storage and dispatches the job to Stage 1.
///
/// # Arguments
/// * `app` - The application state
/// * `job_id` - The full job ID (format: "{user_id}_{job_index}")
/// * `user_id` - The user ID
/// * `front_file` - The front video file
/// * `side_file` - The side video file
/// * `age` - Patient age for metadata
/// * `sex` - Patient sex for metadata
async fn upload_and_dispatch(
    app: Arc<AppState>,
    job_id: &str,
    user_id: &str,
    front_file: UploadRequestFile,
    side_file: UploadRequestFile,
    age: i16,
    sex: char,
) -> Result<()> {
    // Extract file extensions
    let front_extension = front_file
        .name
        .rsplit('.')
        .next()
        .context("Front file must have an extension!")?;
    let side_extension = side_file
        .name
        .rsplit('.')
        .next()
        .context("Side file must have an extension!")?;

    // Build storage paths
    let front_key = StoragePaths::upload_front_video(job_id, front_extension);
    let side_key = StoragePaths::upload_side_video(job_id, side_extension);

    println!("Uploading front video to: {}", front_key);
    let _: () = app.storage
        .upload(&front_key, front_file.bytes.to_vec(), Some("video/mp4"))
        .await
        .context("Failed to upload front video to Firebase Storage!")?;

    println!("Uploading side video to: {}", side_key);
    let _: () = app.storage
        .upload(&side_key, side_file.bytes.to_vec(), Some("video/mp4"))
        .await
        .context("Failed to upload side video to Firebase Storage!")?;

    println!("Files uploaded successfully, dispatching to Stage 1...");

    // Build the stage 1 request
    let mut input_keys = HashMap::new();
    input_keys.insert("front_video".to_string(), front_key);
    input_keys.insert("side_video".to_string(), side_key);

    let callback_url = std::env::var("BACKEND_CALLBACK_URL")
        .unwrap_or_else(|_| "http://localhost:3000/api/v1/webhook/stage/1".to_string());

    let stage_request = StageJobRequest {
        job_id: job_id.to_string(),
        user_id: user_id.to_string(),
        stage: StageNumber::Stage1MediaConversion,
        callback_url,
        input_keys,
        metadata: JobMetadata {
            age: Some(age),
            sex: Some(sex),
            extra: HashMap::new(),
        },
    };

    // Dispatch to Stage 1 microservice
    let stage1_url = std::env::var("STAGE1_SERVICE_URL")
        .unwrap_or_else(|_| "http://localhost:8001/submit".to_string());

    let client = reqwest::Client::new();
    let response: reqwest::Response = client
        .post(&stage1_url)
        .json(&stage_request)
        .send()
        .await
        .context("Failed to connect to Stage 1 service!")?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(anyhow!(
            "Stage 1 service returned error {}: {}",
            status,
            body
        ));
    }

    println!("Job {} dispatched to Stage 1 successfully!", job_id);
    Ok(())
}
