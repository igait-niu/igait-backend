use crate::{ 
    database::{ Database, Status },
    request::query_metis,
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

pub async fn work_queue(app: Arc<Mutex<AppState>>) {
    loop {
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

                let try_status = app.lock().await
                    .db
                    .get_status(
                        user_id.to_string(), 
                        job_id.parse::<usize>().expect("File had invalid job ID!")
                    ).await; 
                    
                match try_status {
                    Some(status) => {
                        match status.code {
                            StatusCode::Processing | StatusCode::Submitting => {},
                            StatusCode::Queue => {
                                println!("\n----- [ State Update ] -----");
                                print_be(&format!("Top option (Job {job_id} for '{user_id}') not processing! Firing inference job request..."));
                                
                                
                                app.lock().await
                                    .db.update_status(
                                        user_id.to_string(),
                                        job_id.parse::<usize>().expect("File had invalid job ID!"),
                                        Status {
                                            code: StatusCode::Processing,
                                            value: String::from("Querying METIS and awaiting response...")
                                        }
                                    ).await;
                                
                                if let Err(reason) = 
                                    query_metis(
                                        user_id.to_string(), job_id.to_string(),
                                        std::env::var("AWS_ACCESS_KEY_ID").expect("MISSING AWS_ACCESS_KEY_ID!"),
                                        std::env::var("AWS_SECRET_ACCESS_KEY").expect("MISSING AWS_SECREt_ACCESS_KEY!"),
                                        std::env::var("IGAIT_ACCESS_KEY").expect("MISSING IGAIT_ACCESS_KEY!")
                                    ).await
                                {
                                    app.lock().await
                                    .db.update_status(
                                        user_id.to_string(),
                                        job_id.parse::<usize>().expect("File had invalid job ID!"),
                                        Status {
                                            code: StatusCode::InferenceErr,
                                            value: format!("Couldn't query METIS for reason {reason}!")
                                        }
                                    ).await;
                                }
                            },
                            _ => {
                                println!("\n----- [ State Update ] -----");
                                print_be(&format!("Unusual status code detected, purging accordingly: {:?}", status.code));

                                // Purge that directory
                                if remove_dir_all(format!("data/queue/{}", dir_name)).await.is_err() {
                                    println!("FAILED TO REMOVE 'data/queue/{}'!", dir_name);
                                };
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
            }
        } else {
            panic!("Failed to read from queue director! Please ensure 'data/queue' exists!");
        }
    }
}