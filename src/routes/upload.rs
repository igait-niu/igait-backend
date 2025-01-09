use std::{sync::Arc, time::SystemTime};

use axum::{body::Bytes, extract::{Multipart, State}};
use tokio::{io::AsyncWriteExt, sync::Mutex};
use anyhow::{ Result, Context, anyhow };

use crate::{
    helper::{
        email::send_welcome_email, lib::{copy_file, metis_qsub, AppError, AppState, Job, JobStatus, JobStatusCode, JobTaskID, SSHPath}, metis::{
            METIS_HOSTNAME, METIS_INPUTS_DIR, METIS_PBS_PATH, METIS_USERNAME
        }
    }, print_be, print_s3,
};

/// The required arguments for the upload request.
struct UploadRequestArguments {
    uid:              String,
    age:              i16,
    ethnicity:        String,
    sex:              char,
    height:           String,
    weight:           i16,
    email:            String,
    front_file:       UploadRequestFile,
    side_file:        UploadRequestFile
}

/// A representation of a file in a `Multipart` request.
struct UploadRequestFile {
    name:  String,
    bytes: Bytes
}


/// Takes in the `Multipart` request and unpacks the arguments into a `UploadRequestArguments` object.
/// 
/// # Fails
/// If any of the fields are missing or if the files are too large.
/// 
/// # Arguments
/// * `multipart` - The `Multipart` object to unpack.
/// * `task_number` - The task number to print out to the console.
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
        front_file: UploadRequestFile {
            name: front_file_name, 
            bytes: front_file_bytes
        },
        side_file: UploadRequestFile {
            name: side_file_name,
            bytes: side_file_bytes
        }
    })
}

/// The entrypoint for the upload request.
/// 
/// # Fails
/// * If the arguments are missing.
/// * If the files are too large.
/// * If the files fail to save to S3.
/// * If the job fails to save to the database.
/// * If the welcome email fails to send.
/// 
/// # Arguments
/// * `app` - The application state.
/// * `multipart` - The `Multipart` object to unpack.
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
    let arguments = unpack_upload_arguments(
            &mut multipart,
            task_number
        ).await
        .context("Failed to unpack arguments!")?;

    // Build a new status object
    let mut status = JobStatus {
        code: JobStatusCode::Submitting,
        value: String::from("")
    };

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
        status:    status.clone(),
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
            arguments.front_file,
            arguments.side_file,
            &arguments.uid, 
            job_id,
            task_number
        ).await 
    {
        // Populate the status object
        status.code = JobStatusCode::SubmissionErr;
        status.value = err.to_string();

        // Update the status of the job
        app.lock().await
            .db.update_status(
                &arguments.uid,
                job_id,
                status,
                task_number
            ).await
            .context("Failed to update the status of the job! It failed to save, however.")?;

        // Early return as a failure
        return Err(AppError(anyhow!("Failed to save files to S3! Error:\n{}", err)));
    }
    
    // Populate the status object
    status.code = JobStatusCode::Queue;
    status.value = String::from("Currently in queue.");

    // Send the welcome email
    send_welcome_email(
        app.clone(),
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
            status,
            task_number
        ).await
        .context("Failed to update the status of the job! However, it was otherwise saved.")?;

    Ok(())
}

/// Saves the upload files to S3 and the local filesystem.
/// 
/// # Fails
/// * If the files fail to save to S3.
/// * If the files fail to save to the local filesystem.
/// 
/// # Arguments
/// * `app` - The application state.
/// * `front_file` - The front file to save.
/// * `side_file` - The side file to save.
/// * `user_id` - The user ID to save the files under.
/// * `job_id` - The job ID to save the files under.
/// * `job` - The job object to save to the local filesystem.
/// * `task_number` - The task number to print out to the console.
async fn save_upload_files<'a> (
    app:              Arc<Mutex<AppState>>,
    front_file:       UploadRequestFile,
    side_file:        UploadRequestFile,
    user_id:          &str,
    job_id:           usize,
    task_number:      JobTaskID
) -> Result<()> {
    // Unpack the extensions
    let front_extension = front_file.name.split(".")
        .nth(1)
        .context("Must have a file extension!")?;
    let side_extension = side_file.name.split(".")
        .nth(1)
        .context("Must have a file extension!")?;
    
    // Ensure a directory exists for this file ID
    let job_file_identifier = format!("{}-{}", user_id, job_id);
    let dir_path = format!("inputs/{}", job_file_identifier);
    if tokio::fs::read_dir(&dir_path).await.is_err() {
        // If it doesn't exist, create it
        tokio::fs::create_dir(&dir_path).await
            .context("Unable to create directory for inputs file!")?;

        print_be!(task_number, "Created directory for inputs file: {dir_path}");
    }

    // Build path ID and file handles
    let front_file_path = format!("{}/{}__F_.{}", dir_path, job_file_identifier, front_extension);
    let side_file_path = format!("{}/{}__S_.{}", dir_path, job_file_identifier, side_extension);
    let mut front_file_handle = tokio::fs::File::create(&front_file_path)
        .await
        .context("Could not create front file!")?;
    let mut side_file_handle = tokio::fs::File::create(&side_file_path)
        .await
        .context("Could not save side file!")?;

    // Write files to the inputs folder
    front_file_handle.write_all(&front_file.bytes.clone())
        .await
        .context("Couldn't write the byte contents of the front video file to a physical file!")?;
    front_file_handle.flush()
        .await
        .context("Unable to flush inputs file!")?;
    side_file_handle.write_all(&side_file.bytes.clone())
        .await
        .context("Couldn't write the byte contents of the side video file to a physical file!")?;
    side_file_handle.flush()
        .await
        .context("Unable to flush inputs file!")?;

    // Build byte vectors
    let mut front_byte_vec: Vec<u8> = Vec::new();
    let mut side_byte_vec: Vec<u8> = Vec::new();
    front_byte_vec.write_all(&front_file.bytes)
        .await
        .context("Failed to build u8 vector from the front file's Bytes object!")?;
    side_byte_vec.write_all(&side_file.bytes)
        .await
        .context("Failed to build u8 vector from side file's Bytes object!")?;

    // Upload the all three files to S3
    app.lock()
        .await
        .bucket
        .put_object(format!("{}/inputs/{}/front.{}", user_id, job_id, front_extension), &front_byte_vec)
        .await 
        .context("Failed to upload front file to S3! Continuing regardless.")?;
    print_s3!(task_number, "Successfully uploaded front file to S3!");
    app.lock()
        .await
        .bucket
        .put_object(format!("{}/inputs/{}/side.{}", user_id, job_id, side_extension), &side_byte_vec)
        .await
        .context("Failed to upload front side to S3! Continuing regardless.")?;
    print_s3!(task_number, "Successfully uploaded side file to S3!");
    print_be!(task_number, "Successfully saved all files physically and to S3!");
    

    // Copy files to Metis
    print_be!(task_number, "Copying files to Metis...");
    copy_file(
        METIS_USERNAME,
        METIS_HOSTNAME,
        SSHPath::Local(&front_file_path),
        SSHPath::Remote(
            &format!(
                "{}/{}__F_.{}",
                METIS_INPUTS_DIR,
                job_file_identifier,
                front_extension
            )
        ),
        false
    ).await
        .context("Couldn't move file from local to Metis!")?;
    copy_file(
        METIS_USERNAME, METIS_HOSTNAME,
        SSHPath::Local(&side_file_path),
        SSHPath::Remote(
            &format!(
                "{}/{}__S_.{}",
                METIS_INPUTS_DIR,
                job_file_identifier,
                side_extension
            )
        ),
        false
    ).await
        .context("Couldn't move file from local to Metis!")?;
    print_be!(task_number, "Successfully copied files to Metis!");

    // Launch the Metis inference
    print_be!(task_number, "Launching PBS batchfile on Metis");
    let pbs_job_id = metis_qsub(
        METIS_USERNAME,
        METIS_HOSTNAME,
        METIS_PBS_PATH,
        vec!("-v", &format!("ID={}", job_file_identifier))
    ).await
        .map_err(|e| anyhow!("Couldn't launch PBS batchfile on Metis! Full error: {e:?}"))?;
    print_be!(task_number, "Successfully launched PBS batchfile! PBS Job ID: '{pbs_job_id}'");

    // Write the job ID to a file
    let pbs_job_id_file_path = format!("{}/pbs_job_id", dir_path);
    let mut pbs_job_id_file_handle = tokio::fs::File::create(&pbs_job_id_file_path)
        .await
        .context("Could not create front file!")?;
    pbs_job_id_file_handle.write_all(&pbs_job_id.as_bytes())
        .await
        .context("Couldn't write the PBS Job ID to a physical file!")?;
    pbs_job_id_file_handle.flush()
        .await
        .context("Unable to flush PBS Job ID file!")?;

    // Return as successful
    Ok(())
}