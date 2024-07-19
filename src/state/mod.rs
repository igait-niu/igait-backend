use crate::{ 
    database::{ Database, Status }, print_be, request::{query_metis, StatusCode}, Arc, Mutex
};

use std::fs::remove_dir_all;

use tokio::{
    time::{
        sleep,
        Duration
    },
    fs::{ 
        read_dir, DirEntry
    }
};
use s3::{ Bucket, creds::Credentials };
use colored::Colorize;
use anyhow::{ anyhow, Context, Result };




#[derive(Debug)]
pub struct AppState {
    pub db: Database,
    pub bucket: Bucket,
    pub task_number: u128
}
impl AppState {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            db: Database::init().await.context("Failed to initialize database while setting up app state!")?,
            bucket: Bucket::new(
                "igait-storage",
                "us-east-2".parse().context("Improper region!")?,
                Credentials::default().context("Couldn't unpack credentials! Make sure that you have set AWS credentials in your system environment.")?,
            ).context("Failed to initialize bucket!")?,
            task_number: 0
        })
    }
}

async fn check_dir(
    app:         Arc<Mutex<AppState>>,
    entry:       &DirEntry
) -> Result<()> {
    // Read dir name to prepare to extract data
    let dir_name = entry.file_name()
        .into_string()
        .map_err(|e| anyhow!("{e:?}"))
        .context("Path is invalid Unicode!")?;

    // Ignore the `.gitignore` file
    if dir_name == ".gitignore" {
        return Ok(());
    }

    // Split it into various chunks to be able to
    //  extract the user and job ID.
    let mut dir_name_chunks = dir_name
        .split(".")
        .next()
        .context("Parser found a malformed file name!")?
        .split("_");

    // Extract the user and job ID from the dir name
    let uid = dir_name_chunks
        .next()
        .context("Must have valid folder name in format '<id>_<job-id>'!")?;
    let job_id = dir_name_chunks
        .next()
        .context("Must have valid folder name in format '<id>_<job-id>'!")?
        .parse::<usize>().context("File had invalid job ID!")?;

    // Ping the database for the status of the job using 
    //  the user ID and job ID.
    let status = app.lock().await
        .db.get_status(
            uid,
            job_id,
            0
        )
        .await
        .or_else(|_| {
            // Purge that directory
            if let Err(e) = remove_dir_all(format!("queue/{}", dir_name)).context("FAILED TO REMOVE") {
                return Err(e);
            }

            Err(anyhow!("\t\tJob didn't exist - Purging files accordingly."))
        })?;

    // If the status is processing or submitting, we don't need to do anything,
    //  the backend is already working on it.
    if status.code == StatusCode::Processing || status.code == StatusCode::Submitting {
        return Ok(());
    }

    // If we're here, we have an unusual status code that we need to handle,
    //  we'll purge the directory and update the status accordingly.
    if status.code == StatusCode::InferenceErr || status.code == StatusCode::SubmissionErr || status.code == StatusCode::Complete {
        print_be!(0, "\n----- [ State Update ] -----");
        let code = status.code;
        print_be!(0, "Unusual status code detected, purging accordingly: {code:#?}");
    
        // Purge that directory
        remove_dir_all(format!("queue/{}", dir_name)).context(format!("FAILED TO REMOVE 'queue/{}'!", dir_name))?;
    }

    // If it's in the queue, and we're at this state in the code, we
    //  can go ahead and post the request to METIS.
    print_be!(0, "\n----- [ State Update ] -----");
    print_be!(0, "Top option (Job {job_id} for '{uid}') not processing! Firing inference job request...");
        
    // Update the status of the job to 'Processing'
    app.lock().await.db.update_status(
            uid,
            job_id,
            Status {
                code: StatusCode::Processing,
                value: String::from("Querying METIS and awaiting response...")
            },
            0
        ).await
        .context("Failed to update status to 'Processing'!")?;
    
    // Query METIS and handle any errors.
    let query_result = query_metis(uid, job_id, 0).await;
    
    // If the query failed, we'll update the status of the job to reflect that.
    if let Err(reason) = query_result {
        app.lock().await.db.update_status(
                uid,
                job_id,
                Status {
                    code: StatusCode::InferenceErr,
                    value: format!("Couldn't query METIS for reason '{reason}'!")
                },
                0
            ).await
            .context("Failed to update status to 'InferenceErr'!")?;

        Err(anyhow!("Couldn't query METIS for reason '{reason}'!"))?
    }

    Ok(())
}

pub async fn work_queue(app: Arc<Mutex<AppState>>) -> Result<()> {
    loop {
        sleep(Duration::from_secs(5)).await;

        let mut dir = read_dir("queue")
            .await
            .context("Failed to read from queue director! Please ensure 'queue' exists!")?;
        
        while let Some(entry) = dir.next_entry().await.context("Failed to read from queue directory!")? {
            if let Err(e) = check_dir(app.clone(), &entry).await {
                print_be!(0, "{e:?}\n\nContinuing as usual...");
            }
        }
    }
}