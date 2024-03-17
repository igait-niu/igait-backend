use tokio::fs::{ File };
use tokio::io::AsyncWriteExt;
use std::time::SystemTime;

use axum::{
    body::{ Bytes },
    extract::{ 
        State, Multipart
    }
};
use tokio::fs::{
    create_dir,
    read_dir
};

use crate::state::{ AppState };
use crate::request::{ StatusCode };
use crate::database::{ Status, Job };
use crate::print::*;
use crate::{
    Arc, Mutex
};

/* Primary Routes */
pub async fn upload(State(app): State<Arc<Mutex<AppState>>>, mut multipart: Multipart) -> Result<(), String> {
    print_be("Recieved request!");

    let mut uid: Option<String> = None;
    let mut age: Option<i16> = None;
    let mut ethnicity: Option<String> = None;
    let mut sex: Option<char> = None;
    let mut height: Option<String> = None;
    let mut weight: Option<i16> = None;
    let mut status = Status {
        code: StatusCode::Submitting,
        value: String::from("")
    };

    let mut front_file_name: Option<String> = None;
    let mut front_file_bytes: Result<Bytes, String> = Err(String::from("File download error!"));

    let mut side_file_name: Option<String> = None;
    let mut side_file_bytes: Result<Bytes, String> = Err(String::from("File download error!"));

    while let Some(field) = multipart
        .next_field().await
        .map_err(|_| {
            String::from("Malformed multipart request!\nThere's a few reasons why this could happen, but it's probably the length of the file.\nhttps://docs.rs/axum/latest/axum/extract/multipart/struct.MultipartError.html")
        })?
    {
        print_be(&format!("Field Incoming: {:?} - File Attached: {:?}", field.name(), field.file_name()));
        match field.name() {
            Some("fileuploadfront") => {
                front_file_name = field
                    .file_name().and_then(|x| Some(String::from(x)));
                front_file_bytes = field.bytes()
                    .await
                    .map_err(|_| {
                        String::from("Could not unpack bytes from field 'fileuploadfront'! Was there no file attached?")
                    });
            },
            Some("fileuploadside") => {
                side_file_name = field
                    .file_name().and_then(|x| Some(String::from(x)));
                side_file_bytes = field.bytes()
                    .await
                    .map_err(|_| {
                        String::from("Could not unpack bytes from field 'fileuploadside'! Was there no file attached?")
                    });
            },
            Some("uid") => {
                uid = Some(
                        field
                            .text().await
                            .map_err(|_| {
                                String::from("Field 'uid' wasn't readable as text!")
                            })?
                            .to_string()
                    );
            }
            Some("age") => {
                age = Some(
                        field
                            .text().await
                            .map_err(|_| {
                                String::from("Field 'age' wasn't readable as text!")
                            })?
                            .parse()
                            .map_err(|_| {
                                String::from("Field 'age' wasn't parseable as a number! Was the entry only digits?")
                            })?
                    );
            },
            Some("ethnicity") => {
                ethnicity = Some(
                        field
                            .text().await
                            .map_err(|_| {
                                String::from("Field 'ethnicity' wasn't readable as text!")
                            })?
                    );
            },
            Some("sex") => {
                sex = Some(
                        field
                            .text().await
                            .map_err(|_| {
                                String::from("Field 'sex' wasn't readable as text!")
                            })?
                            .chars()
                            .nth(0)
                            .ok_or(String::from("Field 'sex' didn't have a vaild entry! Was it empty?"))?
                    );
            },
            Some("height") => {
                height = Some(
                        field
                            .text().await
                            .map_err(|_| {
                                String::from("Field 'height' wasn't readable as text!")
                            })?
                    );
            },
            Some("weight") => {
                weight = Some(
                        field
                            .text().await
                            .map_err(|_| {
                                String::from("Field 'weight' wasn't readable as text!")
                            })?
                            .parse()
                            .map_err(|_| {
                                String::from("Field 'weight' wasn't parseable as a number! Was the entry only digits?")
                            })?
                    );
            },
            _ => {
                print_be("Which had an unknown/no field name...");
            }
        }
    }

    let job_id = app.lock().await
        .db
        .count_jobs(
            String::from(uid.clone().ok_or("Missing 'uid' in request!")?)
        ).await;

    let built_job = Job {
        age:        age.ok_or("Missing 'age' in request!")?,
        ethnicity:  ethnicity.ok_or("Missing 'ethnicity' in request!")?,
        sex:        sex.ok_or("Missing 'sex' in request!")?,
        height:     height.ok_or("Missing 'height' in request!")?,
        weight:     weight.ok_or("Missing 'weight' in request!")?,
        status:     status.clone(),
        timestamp:  SystemTime::now(),
    };
    
    // I'm aware that this .ok_or is
    // redundant and unreachable.
    app.lock().await
        .db.new_job(uid.clone().ok_or("Missing 'uid' in request!")?, built_job).await;

    match save_files( 
            app.clone(),
            front_file_name.clone(),
            front_file_bytes.clone(),
            side_file_name.clone(),
            side_file_bytes.clone(),
            uid.clone().ok_or("Missing 'uid' in request!")?, 
            job_id.to_string()
        ).await
    {
        Ok(code) => {
            status.code = code;
            status.value = String::from("Currently in queue.");
        },
        Err(err_msg) => {
            status.code = StatusCode::SubmissionErr;
            status.value = err_msg;
        }
    }

    // I'm aware that this .ok_or is
    // redundant and unreachable.
    app.lock().await
        .db.update_status(
            uid.ok_or("Missing 'uid' in request!")?,
            job_id,
            status).await;

    Ok(())
}
async fn save_files<'a> (
    app: Arc<Mutex<AppState>>,
    _front_file_name: Option<String>,
    _front_file_bytes: Result<Bytes, String>, 
    _side_file_name: Option<String>,
    _side_file_bytes: Result<Bytes, String>, 
    user_id: String,
    job_id: String
) -> Result<StatusCode, String> {
    // Unpack the file names
    let front_file_name = _front_file_name
        .ok_or_else(|| {
            String::from("Must have associated file name in multipart!")
        })?;
    let side_file_name = _side_file_name
        .ok_or_else(|| {
            String::from("Must have associated file name in multipart!")
        })?;
    
    // Unpack the extension
    let front_extension = front_file_name.split(".")
        .nth(1)
        .ok_or_else(|| {
            String::from("Must have a file extension!")
        })?;
    let side_extension = side_file_name.split(".")
        .nth(1)
        .ok_or_else(|| {
            String::from("Must have a file extension!")
        })?;

    // Unpack the data
    let front_data = _front_file_bytes?;
    let side_data = _side_file_bytes?;

    // Ensure a directory exists for this file ID
    let dir_path = format!("data/queue/{}_{}", user_id, job_id);
    if read_dir(&dir_path).await.is_err() {
        create_dir(&dir_path).await
            .map_err(|_| String::from("Unable to create directory for queue file!"))?;
    }

    // Build path ID and file handle
    let queue_front_file_path = format!("{}/front.{}", dir_path, front_extension);
    let queue_side_file_path = format!("{}/side.{}", dir_path, side_extension);
    let mut queue_front_file_handle = File::create(queue_front_file_path)
        .await
        .map_err(|_| String::from("Unable to open queue file!"))?;
    let mut queue_side_file_handle = File::create(queue_side_file_path)
        .await
        .map_err(|_| String::from("Unable to open queue file!"))?;

    // Write data
    queue_front_file_handle.write_all(&front_data.clone())
        .await
        .map_err(|_| String::from("Unable to write queue file!"))?;
    queue_front_file_handle.flush()
        .await
        .map_err(|_| String::from("Unable to flush queue file!"))?;
    queue_side_file_handle.write_all(&side_data.clone())
        .await
        .map_err(|_| String::from("Unable to write queue file!"))?;
    queue_side_file_handle.flush()
        .await
        .map_err(|_| String::from("Unable to flush queue file!"))?;

    let mut front_byte_vec: Vec<u8> = Vec::new();
    front_byte_vec.write_all(&front_data).await
        .map_err(|_| String::from("Failed to build u8 vector from Bytes!"))?;
    let mut side_byte_vec: Vec<u8> = Vec::new();
    side_byte_vec.write_all(&side_data).await
        .map_err(|_| String::from("Failed to build u8 vector from Bytes!"))?;

    match 
        app.lock()
            .await
            .bucket
            .put_object(format!("{}/{}/front.{}", user_id, job_id, front_extension), &front_byte_vec)
            .await 
    {
        Ok(_) => print_s3("Successfully uploaded front file to S3!"),
        _ => print_s3("Failed to upload front file to S3! Continuing regardless.")
    }

    match
        app.lock()
            .await
            .bucket
            .put_object(format!("{}/{}/side.{}", user_id, job_id, side_extension), &side_byte_vec)
            .await
    {
        Ok(_) => print_s3("Successfully uploaded front file to S3!"),
        _ => print_s3("Failed to upload front file to S3! Continuing regardless.")
    }
    
    Ok(StatusCode::Queue)
}