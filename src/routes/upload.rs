use std::{sync::Arc, time::SystemTime};

use axum::{body::Bytes, extract::{Multipart, State}};
use tokio::{io::AsyncWriteExt, sync::Mutex};
use anyhow::{ Result, Context, anyhow };

use crate::{helper::{email::send_welcome_email, lib::{AppError, AppState, Job, JobStatus, JobStatusCode, JobTaskID}}, print_be, print_s3};

struct UploadRequestArguments {
    uid:              String,
    age:              i16,
    ethnicity:        String,
    sex:              char,
    height:           String,
    weight:           i16,
    email:            String,
    status:           JobStatus,
    front_file_name:  String,
    side_file_name:   String,
    front_file_bytes: Bytes,
    side_file_bytes:  Bytes
}
async fn unpack_upload_arguments(
    multipart:   &mut Multipart,
    task_number: JobTaskID
) -> Result<UploadRequestArguments> {
    // Initialize all of the fields as options
    let mut uid_option:       Option<String> = None;
    let mut age_option:       Option<i16>    = None;
    let mut ethnicity_option: Option<String> = None;
    let mut sex_option:       Option<char>   = None;
    let mut height_option:    Option<String> = None;
    let mut weight_option:    Option<i16>    = None;
    let mut email_option:     Option<String> = None;

    // Build a new status object
    let status = JobStatus {
        code: JobStatusCode::Submitting,
        value: String::from("")
    };

    // Initialize the file fields as options
    let mut front_file_name_option:  Option<String> = None;
    let mut side_file_name_option:   Option<String> = None;
    let mut front_file_bytes_option: Option<Bytes>  = None;
    let mut side_file_bytes_option:  Option<Bytes>  = None;

    // Loop through the fields
    while let Some(field) = multipart
        .next_field().await
        .context("Bad uploadrequest! Is it possible you submitted a file over the size limit?")?
    {
        let name = field.name();
        let field_name = field.file_name();
        print_be!(task_number, "Field Incoming: {name:#?} - File Attached: {field_name:?}");
        
        match field.name() {
            Some("fileuploadfront") => {
                front_file_name_option = field
                    .file_name().and_then(|x| Some(String::from(x)));
                front_file_bytes_option = Some(field.bytes()
                    .await
                    .context("Could not unpack bytes from field 'fileuploadfront'! Was there no file attached?")?);
            },
            Some("fileuploadside") => {
                side_file_name_option = field
                    .file_name().and_then(|x| Some(String::from(x)));
                side_file_bytes_option = Some(field.bytes()
                    .await
                    .context("Could not unpack bytes from field 'fileuploadside'! Was there no file attached?")?);
            },
            Some("uid") => {
                uid_option = Some(
                        field
                            .text().await
                            .context("Field 'uid' wasn't readable as text!")?
                            .to_string());
            }
            Some("age") => {
                age_option = Some(
                        field
                            .text().await
                            .context("Field 'age' wasn't readable as text!")?
                            .parse()
                            .context("Field 'age' wasn't parseable as a number! Was the entry only digits?")?);
            },
            Some("ethnicity") => {
                ethnicity_option = Some(
                        field
                            .text().await
                            .context("Field 'ethnicity' wasn't readable as text!")?);
            },
            Some("email") => {
                email_option = Some(
                        field
                            .text().await
                            .context("Field 'email' wasn't readable as text!")?);
            },
            Some("sex") => {
                sex_option = Some(
                        field
                            .text().await
                            .context("Field 'sex' wasn't readable as text!")?
                            .chars()
                            .nth(0)
                            .context("Field 'sex' didn't have a vaild entry! Was it empty?")?
                    );
            },
            Some("height") => {
                height_option = Some(
                        field
                            .text().await
                            .context("Field 'height' wasn't readable as text!")?);
            },
            Some("weight") => {
                weight_option = Some(
                        field
                            .text().await
                            .context("Field 'weight' wasn't readable as text!")?
                            .parse()
                            .context("Field 'weight' wasn't parseable as a number! Was the entry only digits?")?);
            },
            _ => {
                print_be!(task_number, "Which had an unknown/no field name...");
            }
        }
    }

    // Make sure all of the fields are present
    let uid: String       = uid_option.ok_or(       anyhow!( "Missing 'uid' in request!"      ))?;
    let age: i16          = age_option.ok_or(       anyhow!( "Missing 'age' in request"       ))?;
    let ethnicity: String = ethnicity_option.ok_or( anyhow!( "Missing 'ethnicity' in request" ))?;
    let sex: char         = sex_option.ok_or(       anyhow!( "Missing 'sex' in request"       ))?;
    let height: String    = height_option.ok_or(    anyhow!( "Missing 'height' in request"    ))?;
    let weight: i16       = weight_option.ok_or(    anyhow!( "Missing 'weight' in request"    ))?;
    let email: String     = email_option.ok_or(     anyhow!( "Missing 'email' in request"     ))?;

    // Make sure all of the file fields are present
    let front_file_name:  String = front_file_name_option.ok_or(  anyhow!( "Missing 'fileuploadfront' in request!" ))?;
    let side_file_name:   String = side_file_name_option.ok_or(   anyhow!( "Missing 'fileuploadside' in request!"  ))?;
    let front_file_bytes: Bytes  = front_file_bytes_option.ok_or( anyhow!( "Missing 'fileuploadfront' in request!" ))?;
    let side_file_bytes:  Bytes  = side_file_bytes_option.ok_or(  anyhow!( "Missing 'fileuploadside' in request!"  ))?;

    Ok(UploadRequestArguments {
        uid, age, ethnicity, sex, height, weight, email, 
        status,
        front_file_name, side_file_name, front_file_bytes, side_file_bytes
    })
}
pub async fn upload_entrypoint(
    State(app): State<Arc<Mutex<AppState>>>,
    mut multipart: Multipart
) -> Result<(), AppError> {
    // Allocate a new task number
    app.lock().await
        .task_number += 1;
    let task_number = app.lock().await.task_number;

    print_be!(task_number, "\n----- [ Recieved base request ] -----");
    print_be!(task_number, "Unpacking arguments...");

    // Unpack the arguments
    let mut arguments = unpack_upload_arguments(
            &mut multipart,
            task_number
        ).await
        .context("Failed to unpack arguments!")?;

    // Generate the new job ID (no need to add 1 since it's 0-indexed)
    let job_id = app.lock().await
        .db
        .count_jobs(
            &arguments.uid,
            task_number
        ).await
        .context("Failed to count the number of jobs!")?;

    // Build the new job object
    let job = Job {
        age:       arguments.age,
        ethnicity: arguments.ethnicity,
        sex:       arguments.sex,
        height:    arguments.height,
        weight:    arguments.weight,
        status:    arguments.status.clone(),
        email:     arguments.email,
        timestamp: SystemTime::now(),
    };
    
    // Add the job to the database
    app.lock().await
        .db.new_job(
            &arguments.uid,
            job.clone(),
            task_number
        ).await
        .context("Failed to add the new job to the database!")?;

    // Try to save the files to S3
    if let Err(err) = 
        save_upload_files( 
            app.clone(),
            arguments.front_file_name,
            arguments.front_file_bytes,
            arguments.side_file_name,
            arguments.side_file_bytes,
            &arguments.uid, 
            job_id,
            job.clone(),
            task_number
        ).await 
    {
        // Populate the status object
        arguments.status.code = JobStatusCode::SubmissionErr;
        arguments.status.value = err.to_string();

        // Update the status of the job
        app.lock().await
            .db.update_status(
                &arguments.uid,
                job_id,
                arguments.status,
                task_number
            ).await
            .context("Failed to update the status of the job! It failed to save, however.")?;

        // Early return as a failure
        return Err(AppError(anyhow!("Failed to save files to S3! Error:\n{}", err)));
    }
    
    // Populate the status object
    arguments.status.code = JobStatusCode::Queue;
    arguments.status.value = String::from("Currently in queue.");

    // Send the welcome email
    send_welcome_email(
        &job,
        &arguments.uid,
        job_id,
        task_number
    ).await.context("Failed to send welcome email!")?;

    // Update the status of the job
    app.lock().await
        .db.update_status(
            &arguments.uid,
            job_id,
            arguments.status,
            task_number
        ).await
        .context("Failed to update the status of the job! However, it was otherwise saved.")?;

    Ok(())
}
async fn save_upload_files<'a> (
    app:              Arc<Mutex<AppState>>,
    front_file_name:  String,
    front_file_bytes: Bytes, 
    side_file_name:   String,
    side_file_bytes:  Bytes, 
    user_id:          &str,
    job_id:           usize,
    job:              Job,
    task_number:      JobTaskID
) -> Result<()> {
    // Unpack the extensions
    let front_extension = front_file_name.split(".")
        .nth(1)
        .context("Must have a file extension!")?;
    let side_extension = side_file_name.split(".")
        .nth(1)
        .context("Must have a file extension!")?;
    
    // Ensure a directory exists for this file ID
    let dir_path = format!("queue/{}_{}", user_id, job_id);
    if tokio::fs::read_dir(&dir_path).await.is_err() {
        // If it doesn't exist, create it
        tokio::fs::create_dir(&dir_path).await
            .context("Unable to create directory for queue file!")?;

        print_be!(task_number, "Created directory for queue file: {dir_path}");
    }

    // Build path ID and file handle
    let queue_file_path = format!("{}/data.json", dir_path);
    let mut queue_side_file_handle = tokio::fs::File::create(queue_file_path)
        .await
        .context("Unable to open queue file!")?;

    // Serialize the job data to soon write to the file
    let job_data = serde_json::to_string(&job)
        .context("Unable to serialize data!")?;

    // Write data
    queue_side_file_handle.write_all(job_data.as_bytes())
        .await
        .context("Unable to write queue file!")?;
    queue_side_file_handle.flush()
        .await
        .context("Unable to flush queue file!")?;

    // Build byte vectors
    let mut front_byte_vec: Vec<u8> = Vec::new();
    front_byte_vec.write_all(&front_file_bytes)
        .await
        .context("Failed to build u8 vector from the front file's Bytes object!")?;
    let mut side_byte_vec: Vec<u8> = Vec::new();
    side_byte_vec.write_all(&side_file_bytes)
        .await
        .context("Failed to build u8 vector from side file's Bytes object!")?;

    // Serialize the data to write to the extensions file
    let serialized_extensions = format!("{{\"front\":\"{front_extension}\",\"side\":\"{side_extension}\"}}");
    
    // Upload the all three files to S3
    app.lock()
        .await
        .bucket
        .put_object(format!("{}/{}/front.{}", user_id, job_id, front_extension), &front_byte_vec)
        .await 
        .context("Failed to upload front file to S3! Continuing regardless.")?;
    print_s3!(task_number, "Successfully uploaded front file to S3!");
    app.lock()
        .await
        .bucket
        .put_object(format!("{}/{}/side.{}", user_id, job_id, side_extension), &side_byte_vec)
        .await
        .context("Failed to upload front side to S3! Continuing regardless.")?;
    print_s3!(task_number, "Successfully uploaded side file to S3!");
    app.lock()
        .await
        .bucket
        .put_object(format!("{}/{}/extensions.json", user_id, job_id), serialized_extensions.as_bytes())
        .await
        .context("Failed to upload front extensions JSON data to S3! Continuing regardless.")?;
    print_s3!(task_number, "Successfully uploaded extensions JSON datafile to S3!");
    
    // Return as successful
    print_be!(task_number, "Successfully saved all files physically and to S3!");
    
    Ok(())
}