use crate::{ 
    database::{ Database, Status },
    inference,
    print::*,

    request::{ StatusCode },
    Arc, Mutex
};
use tokio::time::{
    sleep,
    Duration
};
use tokio::fs::{ 
    read_dir,
    remove_dir_all
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
                "igait-storage",
                "us-east-2".parse().expect("Improper region!"),
                Credentials::default().expect("Couldn't unpack credentials!"),
            ).expect("Failed to initialize bucket!")
        }
    }
}

pub async fn work_queue(s: Arc<Mutex<AppState>>) {
    'main: loop {
        sleep(Duration::from_secs(5)).await;

        if let Ok(mut dir) = read_dir("data/queue").await {
            while let Ok(Some(entry)) = dir.next_entry().await {
                // Read dir name to prepare to extract data
                let dir_name = entry.file_name()
                    .into_string().expect("Path is invalid Unicode!");
                let mut dir_name_chunks = dir_name
                    .split(".")
                    .next().expect("Malformed file name!")
                    .split("_");

                // Extract data from dir name
                let user_id = dir_name_chunks
                    .next().expect("Must have valid folder name in format '<id>_<job-id>'!");
                let job_id = dir_name_chunks
                    .next().expect("Must have valid folder name in format '<id>_<job-id>'!");

                print_be(&format!("Checking User {}, Job {}...", user_id, job_id));

                let try_status = s.lock().await
                    .db
                    .get_status(
                        user_id.to_string(), 
                        job_id.parse::<usize>().expect("File had invalid job ID!")
                    ).await; 
                    
                match try_status {
                    Some(status) => {
                        match status.code {
                            StatusCode::Processing => {
                                continue 'main;
                            },
                            StatusCode::Queue => {
                                print_be("Top option not processing! Firing inference job request...")
                            },
                            _ => {
                                println!("Unusual status code detected: {:?}", status.code);
                                continue 'main;
                            }
                        }
                    },
                    _ => {
                        print_be("\t\tJob didn't exist - Purging files accordingly.");

                        // Purge that directory
                        if remove_dir_all(format!("data/queue/{}", dir_name)).await.is_err() {
                            println!("FAILED TO REMOVE 'data/queue/{}'!", dir_name);
                        };
                    }
                }

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
                        
                // Try to grab the front and side file extensions
                if let Ok(mut job_dir) = read_dir(format!("data/queue/{}", dir_name)).await {
                    while let Ok(Some(entry)) = job_dir.next_entry().await {
                        let file_name = entry.file_name()
                            .into_string().expect("Path is invalid Unicode!");
                        
                        if file_name != "data.json" {
                            print_be(&format!("Warning - Unusual file presence in {user_id} - Job {job_id}"));
                        }
                    }
                }

                if let Err(reason) = 
                    inference::run_inference(
                        format!("{user_id}_{job_id}")
                    ).await
                { 
                    s.lock().await
                        .db
                        .update_status(
                            user_id.to_string(), 
                            job_id.parse::<usize>().expect("File had invalid job ID!"), 
                            Status { 
                                code: StatusCode::InferenceErr, 
                                value: reason
                            } )
                        .await;
                }
            }
        } else {
            panic!("Failed to read from queue director! Please ensure 'data/queue' exists!");
        }
    }
}