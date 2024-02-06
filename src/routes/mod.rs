use tokio::fs::{ File };
use tokio::io::AsyncWriteExt;
use std::time::SystemTime;

use std::borrow::BorrowMut;

use sha256::digest;

use axum::{
    body::{ Bytes },
    extract::{ 
        State, Multipart, Path,
        multipart::Field
    }
};

use crate::state::{ AppState };
use crate::request::{ StatusCode };
use crate::database::{ Status, Job };
use crate::{
    Arc, Mutex
};

/* Primary Routes */
pub async fn upload(State(app): State<Arc<Mutex<AppState>>>, mut multipart: Multipart) {
    let mut job_id: Option<usize> = None;
    let mut email: Option<String> = None;
    let mut email_digest: Option<String> = None;
    let mut age: Option<i16> = None;
    let mut ethnicity: Option<String> = None;
    let mut gender: Option<char> = None;
    let mut height: Option<String> = None;
    let mut weight: Option<i16> = None;
    let mut status = Status {
        code: StatusCode::Submitting,
        value: String::from("")
    };

    let mut file_id: Option<String> = None;
    let mut file_name: Option<String> = None;
    let mut file_bytes: Result<Bytes, String> = Err(String::from("File download error!"));

    while let Some(field) = multipart
        .next_field().await
        .map_err(|_| {
            String::from("Malformed multipart request")
        }).unwrap()
    {
        match field.name().unwrap() {
            "fileupload" => {
                file_name = field
                    .file_name().and_then(|x| Some(String::from(x)));
                file_bytes = field.bytes()
                    .await
                    .map_err(|_| {
                        String::from("Could not unwrap bytes!")
                    }).clone();

            },
            "email" => {
                email = Some(field.text().await.unwrap().to_string());
                email_digest = Some(digest(email.clone().unwrap()));
            }
            "age" => {
                age = Some(field.text().await.unwrap().parse().unwrap());
            },
            "ethnicity" => {
                ethnicity = Some(field.text().await.unwrap());
            },
            "gender" => {
                gender = Some(field.text().await.unwrap()
                    .chars()
                    .nth(0).unwrap());
            },
            "height" => {
                height = Some(field.text().await.unwrap());
            },
            "weight" => {
                weight = Some(field.text().await.unwrap()
                    .parse().unwrap());
            },
            _ => {}

        }
    }

    job_id = Some(app.lock().await
        .db
        .count_jobs(String::from(email.clone().expect("Missing email in request!"))).await);
    
    file_id = Some(
        format!("{}_{}",
            email_digest.expect("Missing email in request!"), 
            job_id.expect("Missing file in request!")
        )
    );
    
    match save_file( 
        file_name.clone(),
        file_bytes.clone(),
        file_id.expect("Missing file in request!")).await {
        Ok(code) => {
            status.code = code;
            status.value = String::from("Currently in queue.");
        },
        Err(err_msg) => {
            status.code = StatusCode::SubmissionErr;
            status.value = err_msg;
        }
    }

    let built_job = Job {
        age: age.unwrap(),
        ethnicity: ethnicity.unwrap(),
        gender: gender.unwrap(),
        height: height.unwrap(),
        weight: weight.unwrap(),
        status,
        timestamp: SystemTime::now(),
    };
    

    app.lock().await
        .db.new_job(email.unwrap(), built_job).await;
}
async fn save_file<'a> ( _file_name: Option<String>, _file_bytes: Result<Bytes, String>, file_id: String ) -> Result<StatusCode, String> {
    let file_name = _file_name
        .ok_or_else(|| {
            String::from("Must have associated file name in multipart!")
        })?;
    let extension = file_name.split(".")
        .nth(1)
        .ok_or_else(|| {
            String::from("Must have a file extension!")
        })?;
    if extension != "mp4" {
        return Err(String::from("Must be of filetype MP4!"));
    }

    let data = _file_bytes?;
    // Build path ID and file handle
    let queue_file_path = format!("data/queue/{}.mp4", file_id);
    let mut queue_file_handle = File::create(queue_file_path)
        .await
        .map_err(|_| String::from("Unable to open queue file!"))?;

    // Write data
    queue_file_handle.write_all(&data.clone())
        .await
        .map_err(|_| String::from("Unable to write queue file!"))?;
    queue_file_handle.flush()
        .await
        .map_err(|_| String::from("Unable to flush queue file!"))?;

    // Build path ID and file handle
    let resources_file_path = format!("data/resources/{}.mp4", file_id);
    let mut resources_file_handle = File::create(resources_file_path)
        .await
        .map_err(|_| String::from("Unable to open resource file!"))?;

    // Write data
    resources_file_handle.write_all(&data)
        .await
        .map_err(|_| String::from("Unable to write resource file!"))?;
    resources_file_handle.flush()
        .await
        .map_err(|_| String::from("Unable to flush resource file!"))?;
    
    Ok(StatusCode::Queue)
}