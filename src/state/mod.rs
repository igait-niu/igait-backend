use crate::{ 
    database::{ Database, Status },
    inference,

    request::{ StatusCode },
    Arc, Mutex
};
use tokio::time::{ sleep, Duration };
use tokio::fs::{ 
    read_dir,
};
use s3::Bucket;
use s3::creds::Credentials;

#[derive(Debug)]
pub struct AppState {
    pub db: Database,
    pub bucket: Bucket
}
impl AppState {
    pub async fn new() -> Self {
        Self {
            db: Database::init().await,
            bucket: Bucket::new(
                "igait-resources",
                "us-east-2".parse().expect("Improper region!"),
                Credentials::default().expect("Couldn't unpack credentials!"),
            ).expect("Failed to initialize bucket!")
        }
    }
}

pub async fn work_queue(s: Arc<Mutex<AppState>>) {
    loop {
        if let Ok(mut dir) = read_dir("data/queue").await {
            while let Ok(Some(entry)) = dir.next_entry().await {
                let file_name = entry.file_name()
                    .into_string().expect("Path is invalid Unicode!");
                let mut file_name_chunks = file_name
                    .split(".")
                    .next().expect("Malformed file name!")
                    .split("_");

                let user_id = file_name_chunks
                    .next().expect("Must have valid file name in format '<id>_<job-id>.mp4'!");
                let job_id = file_name_chunks
                    .next().expect("Must have valid file name in format '<id>_<job-id>.mp4'!");

                println!("Working User {}, Job {}", user_id, job_id);
                s.lock().await
                    .db
                    .update_status(
                        user_id.to_string(), 
                        job_id.parse::<usize>().expect("File had invalid job ID!"), 
                        Status { 
                            code: StatusCode::Processing, 
                            value: String::from("Please wait...")
                        } )
                    .await;
                match inference::run_inference(format!("{user_id}_{job_id}")).await {
                    Ok(confidence) => {
                        println!("Completed with confidence {}", confidence);
                        s.lock().await
                            .db
                            .update_status(
                                user_id.to_string(), 
                                job_id.parse::<usize>().expect("File had invalid job ID!"), 
                                Status { 
                                    code: StatusCode::Complete, 
                                    value: confidence.to_string()
                                } )
                            .await;
                    },
                    Err(err_msg) => {
                        println!("Failed with error '{err_msg}'");
                        s.lock().await
                            .db
                            .update_status(
                                user_id.to_string(), 
                                job_id.parse::<usize>().expect("File had invalid job ID!"), 
                                Status { 
                                    code: StatusCode::InferenceErr, 
                                    value: err_msg
                                } )
                            .await;
                    }
                }
            }
        } else {
            panic!("Failed to read from queue director! Please ensure 'data/queue' exists!");
        }

        sleep(Duration::from_secs(5)).await;
    }
}