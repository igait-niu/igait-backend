use std::fs::{ File };
use std::io::Write;
use std::time::SystemTime;

use sha256::digest;

use axum::extract::{ 
    State, Multipart, Path,
    multipart::Field
};

use crate::state::{ AppState };
use crate::request::{ StatusCode };
use crate::database::{ Status, Job };
use crate::{
    Arc, Mutex
};

/* Primary Routes */
pub async fn upload(State(app): State<Arc<Mutex<AppState>>>, mut multipart: Multipart) {
    let email = "todointakeemail@gmail.com";
    let email_digest = digest(email);
    let job_id = app.lock().await
        .db
        .count_jobs(String::from(email)).await;

    let mut age: Option<i16> = None;
    let mut ethnicity: Option<String> = None;
    let mut gender: Option<char> = None;
    let mut height: Option<String> = None;
    let mut weight: Option<i16> = None;
    let mut status = Status {
        code: StatusCode::Submitting,
        value: String::from("")
    };

    while let Some(field) = multipart
        .next_field().await
        .map_err(|_| {
            String::from("Malformed multipart request")
        }).unwrap()
    {
        match field.name().unwrap() {
            "fileupload" => {
                let file_id = format!("{email_digest}_{job_id}");
                match save_file( field, file_id ).await {
                    Ok(code) => {
                        status.code = code;
                        status.value = String::from("Currently in queue.");
                        println!("GOOOOD");
                    },
                    Err(err_msg) => {
                        status.code = StatusCode::SubmissionErr;
                        status.value = err_msg;
                    }
                }
            },
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

    let built_job = Job {
        age: age.unwrap(),
        ethnicity: ethnicity.unwrap(),
        gender: gender.unwrap(),
        height: height.unwrap(),
        weight: weight.unwrap(),
        status,
        timestamp: SystemTime::now(),
    };

    //println!("{:?}", built_job);

    app.lock().await
        .db.new_job(String::from(email), built_job).await;
}
async fn save_file<'a> ( mut field: Field<'a>, file_id: String ) -> Result<StatusCode, String> {
    println!(":3~");
    let file_name = field
        .file_name()
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

    // Grab byte data
    let data = field.bytes()
        .await
        .map_err(|_| {
            String::from("Could not unwrap bytes!")
        })?;

    // Build path ID and file handle
    let queue_file_path = format!("data/queue/{}.mp4", file_id);
    let mut queue_file_handle = File::create(queue_file_path)
        .unwrap();

    // Write data
    queue_file_handle.write_all(&data.clone())
        .map_err(|_| String::from("Unable to write file!"))?;

    // Build path ID and file handle
    let resources_file_path = format!("data/resources/{}.mp4", file_id);
    let mut resources_file_handle = File::create(resources_file_path)
        .unwrap();

    // Write data
    resources_file_handle.write_all(&data)
        .map_err(|_| String::from("Unable to write file!"))?;
    
    Ok(StatusCode::Queue)
}