use crate::{ 
    database::{ Database, Status, Job },
    inference,

    request::{ StatusCode },
    Arc, Mutex
};
use std::time::SystemTime;
use tokio::time::{ sleep, Duration };
use tokio::fs::{ 
    read_dir,
};

#[derive(Debug)]
pub struct AppState {
    pub db: Database
}
impl AppState {
    pub async fn new() -> Self {
        Self {
            db: Database::init().await
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

                match inference::run_inference(format!("{user_id}_{job_id}")).await {
                    Ok(confidence) => {
                        println!("{confidence}");
                    },
                    Err(err_msg) => {
                        println!("{err_msg}");
                    }
                }
                println!("Working User {}, Job {}", user_id, job_id);
            }
        } else {
            panic!("Failed to read from queue director! Please ensure 'data/queue' exists!");
        }

        sleep(Duration::from_secs(5)).await;
    }
}