use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use std::time::SystemTime;
use anyhow::{ Result, Context, anyhow };

use axum::{
    body::{Body, Bytes},
    extract::{ Multipart, State}, response::{IntoResponse, Response}
};
use tokio::fs::{ create_dir, read_dir };
use chrono::{ DateTime, Utc };

use crate::{
    email::{
        send_failure_email, send_success_email, send_welcome_email
    }, 
    state::AppState
};
use crate::request::StatusCode;
use crate::database::{ Status, Job };
use crate::print::*;
use crate::{ Arc, Mutex };
use serde_json::Value;

/* 
    The purpose of this interface is to allow our routes to use anyhow's 
     error handling system to return errors in a way that can be easily
     converted into a response. This is done by implementing the IntoResponse
     trait for the AppError struct, which is a wrapper around anyhow::Error.
 */
#[derive(Debug)]
pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response<Body> {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

pub async fn completion(State(app): State<Arc<Mutex<AppState>>>, mut multipart: Multipart) -> Result<&'static str, AppError> {
    println!("\n----- [ Recieved completion update ] -----");

    let mut uid_option: Option<String> = None;
    let mut job_id_option: Option<usize> = None;
    let mut status_code_option: Option<String> = None;
    let mut status_content_option: Option<String> = None;
    let mut igait_access_key_option: Option<String> = None;

    while let Some(field) = multipart
        .next_field().await
        .context("Bad request! Is it possible you submitted a file over the size limit?")?
    {
        print_be(&format!("Field Incoming: {:?}", field.name()));
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
                print_be("Which had an unknown/no field name...");
            }
        }
    }

    // Make sure all of the fields are present
    let uid = uid_option.ok_or(anyhow!("Missing 'user_id' in request!"))?;
    let job_id = job_id_option.ok_or(anyhow!("Missing 'job_id' in request!"))?;
    let status_code = status_code_option.ok_or(anyhow!("Missing 'status_code' in request!"))?;
    let status_content = status_content_option.ok_or(anyhow!("Missing 'status_content' in request!"))?;
    let igait_access_key = igait_access_key_option.ok_or(anyhow!("Missing 'igait_access_key' in request!"))?;

    // First, check the access key against the environment
    if igait_access_key != std::env::var("IGAIT_ACCESS_KEY").context("MISSING 'IGAIT_ACCESS_KEY' in environment!")? {
        print_be("Invalid access key!");
        Err(anyhow!("Invalid access key from completion endpoint!"))?
    }

    // Build a new status object
    let mut status = Status {
        code: StatusCode::Submitting,
        value: status_content.clone()
    };

    // Grab the job it references
    let job: Job = app.lock().await
        .db
        .get_job(
            &uid,
            job_id
        ).await
        .context("The job targeted by the completion request doesn't exist!")?; 

    // Extract the email address and timestamp
    let recipient_email_address = job.email.clone();
    let dt_timestamp_utc: DateTime<Utc> = job.timestamp.clone().into();

    if &status_code == "OK" {
        // This is a success, the inference process was completed
        print_be("Job successful!");
        status.code = StatusCode::Complete;

        // Extract the bytes from the extensions file
        let bytes: Vec<u8> = app.lock()
            .await
            .bucket
            .get_object(
                &format!("{}/{}/extensions.json",
                    uid,
                    job_id
                )
            ).await
            .context("Failed to get extensions file!")?
            .to_vec();

        // Convert the raw bytes to a string
        let extensions_as_string: String = String::from_utf8(bytes)
            .context("There was invalid UTF8 data from the `extensions.json` file!")?;

        // Parse the string into a JSON object
        let extensions: Value = serde_json::from_str(&extensions_as_string)
            .context("Couldn't convert the `extensions.json` file to a JSON object!")?;

        // Generate the presigned URLs
        let front_keyframed_url = app.lock()
                .await
                .bucket
                .presign_get(format!("{}/{}/front_keyframed.{}", uid, job_id, extensions["front"].as_str().context("Invalid extension type for the front file!")?), 86400 * 7, None)
                .expect("Failed to get the front keyframed URL!");
        let side_keyframed_url = app.lock()
                .await
                .bucket
                .presign_get(format!("{}/{}/side_keyframed.{}", uid, job_id, extensions["side"].as_str().context("Invalid extension type for the side file!")?), 86400 * 7, None)
                .expect("Failed to get the side keyframed URL!");

        // Send the success email
        send_success_email(
            &recipient_email_address,
            &status,
            &dt_timestamp_utc,
            &job,
            &front_keyframed_url,
            &side_keyframed_url,
            &uid,
            job_id
        ).await.context("Failed to send success email!")?;
    } else if &status_code == "ERR" {
        // This is a failure, usually due to an error in the inference process
        print_be(&format!("Job unsuccessful - status content: '{status_content}'"));
        status.code = StatusCode::InferenceErr;

        // Send the failure email
        send_failure_email(
            &recipient_email_address,
            &status,
            &dt_timestamp_utc,
            &uid,
            job_id
        ).await.context("Failed to send failure email!")?;
    } else {
        // This is an invalid status code, probably a mistake or bad actor
        print_be("Invalid status code!");
        return Err(AppError(anyhow!("Invalid status code from completion endpoint!")));
    }
    
    Ok("OK")
}
pub async fn upload(State(app): State<Arc<Mutex<AppState>>>, mut multipart: Multipart) -> Result<(), AppError> {
    println!("\n----- [ Recieved base request ] -----");

    // Initialize all of the fields as options
    let mut uid_option:       Option<String> = None;
    let mut age_option:       Option<i16>    = None;
    let mut ethnicity_option: Option<String> = None;
    let mut sex_option:       Option<char>   = None;
    let mut height_option:    Option<String> = None;
    let mut weight_option:    Option<i16>    = None;
    let mut email_option:     Option<String> = None;

    // Build a new status object
    let mut status = Status {
        code: StatusCode::Submitting,
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
        print_be(&format!("Field Incoming: {:?} - File Attached: {:?}", field.name(), field.file_name()));
        
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
                print_be("Which had an unknown/no field name...");
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

    // Generate the new job ID (no need to add 1 since it's 0-indexed)
    let job_id = app.lock().await
        .db
        .count_jobs(
            &uid
        ).await
        .context("Failed to count the number of jobs!")?;

    // Build the new job object
    let job = Job {
        age,
        ethnicity,
        sex,
        height,
        weight,
        status: status.clone(),
        email,
        timestamp:  SystemTime::now(),
    };
    
    // Add the job to the database
    app.lock().await
        .db.new_job( &uid, job.clone() ).await
        .context("Failed to add the new job to the database!")?;

    // Try to save the files to S3
    if let Err(err) = 
        save_files( 
            app.clone(),
            front_file_name,
            front_file_bytes,
            side_file_name,
            side_file_bytes,
            &uid, 
            job_id,
            job.clone()
        ).await 
    {
        // Populate the status object
        status.code = StatusCode::SubmissionErr;
        status.value = err.to_string();

        // Update the status of the job
        app.lock().await
            .db.update_status(
                &uid,
                job_id,
                status
            ).await
            .context("Failed to update the status of the job! It failed to save, however.")?;

        // Early return as a failure
        return Err(AppError(anyhow!("Failed to save files to S3! Error:\n{}", err)));
    }
    
    // Populate the status object
    status.code = StatusCode::Queue;
    status.value = String::from("Currently in queue.");

    // Send the welcome email
    send_welcome_email(
        &job,
        &uid,
        job_id
    ).await.context("Failed to send welcome email!")?;

    // Update the status of the job
    app.lock().await
        .db.update_status(
            &uid,
            job_id,
            status
        ).await
        .context("Failed to update the status of the job! However, it was otherwise saved.")?;

    Ok(())
}


async fn save_files<'a> (
    app: Arc<Mutex<AppState>>,
    front_file_name: String,
    front_file_bytes: Bytes, 
    side_file_name: String,
    side_file_bytes: Bytes, 
    user_id: &str,
    job_id: usize,
    job: Job
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
    if read_dir(&dir_path).await.is_err() {
        // If it doesn't exist, create it
        create_dir(&dir_path).await
            .context("Unable to create directory for queue file!")?;

        print_be(&format!("Created directory for queue file: {}", dir_path));
    }

    // Build path ID and file handle
    let queue_file_path = format!("{}/data.json", dir_path);
    let mut queue_side_file_handle = File::create(queue_file_path)
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
    print_s3("Successfully uploaded front file to S3!");
    app.lock()
        .await
        .bucket
        .put_object(format!("{}/{}/side.{}", user_id, job_id, side_extension), &side_byte_vec)
        .await
        .context("Failed to upload front side to S3! Continuing regardless.")?;
    print_s3("Successfully uploaded side file to S3!");
    app.lock()
        .await
        .bucket
        .put_object(format!("{}/{}/extensions.json", user_id, job_id), serialized_extensions.as_bytes())
        .await
        .context("Failed to upload front extensions JSON data to S3! Continuing regardless.")?;
    print_s3("Successfully uploaded extensions JSON datafile to S3!");
    
    // Return as successful
    print_be("Successfully saved all files physically and to S3!");
    Ok(())
}