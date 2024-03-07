use crate::{ 
    database::{ Database, Status },
    inference,
    print::*,

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
                "igait-storage",
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
                // Read dir name to prepare to extract data
                let dir_name = entry.file_name()
                    .into_string().expect("Path is invalid Unicode!");
                let mut dir_name_chunks = dir_name
                    .split(".")
                    .next().expect("Malformed file name!")
                    .split("_");

                // Extract data from dir name
                let user_id = dir_name_chunks
                    .next().expect("Must have valid file name in format '<id>_<job-id>.mp4'!");
                let job_id = dir_name_chunks
                    .next().expect("Must have valid file name in format '<id>_<job-id>.mp4'!");

                print_be(&format!("Working User {}, Job {}", user_id, job_id));

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
                
                let mut front_file_ext: Option<String> = None;
                let mut side_file_ext: Option<String> = None;

                // Try to grab the front and side file extensions
                if let Ok(mut job_dir) = read_dir(format!("data/queue/{}", dir_name)).await {
                    while let Ok(Some(entry)) = job_dir.next_entry().await {
                        let file_name = entry.file_name()
                            .into_string().expect("Path is invalid Unicode!");
                        let mut file_name_chunks = file_name
                            .split(".");
                        
                        match
                            file_name_chunks
                                .next()
                                .expect("Must have file name!")
                        {
                            "front" => { front_file_ext = Some(file_name_chunks.next().expect("Must have extension!").to_string()); },
                            "side" => { side_file_ext = Some(file_name_chunks.next().expect("Must have extension!").to_string()); },
                            _ => { println!("Warning - unusual file presence '{}'", file_name); }
                        }
                    }
                }

                print_be(&format!("File Extensions: [{:?} {:?}]", front_file_ext, side_file_ext));

                match 
                    inference::run_inference(
                        format!("{user_id}_{job_id}"),
                        front_file_ext.expect("Must have a front file!"),
                        side_file_ext.expect("Must have a side file!")
                    ).await 
                {
                    Ok(confidence) => {
                        print_be(&format!("Completed with confidence {confidence}"));
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
                        print_be(&format!("Failed with error '{err_msg}'"));
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