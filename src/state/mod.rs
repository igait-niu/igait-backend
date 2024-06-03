use anyhow::{ anyhow, Context, Result };

use crate::{ 
    database::{ Database, Status },
    request::query_metis,
    print::*,

    request::StatusCode,
    Arc, Mutex
};
use tokio::time::{
    sleep,
    Duration
};
use std::fs::remove_dir_all;
use tokio::fs::{ 
    read_dir, DirEntry
};
use s3::Bucket;
use s3::creds::Credentials;

#[derive(Debug)]
pub struct AppState {
    pub db: Database,
    pub bucket: Bucket
}
impl AppState {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            db: Database::init().await,
            bucket: Bucket::new(
                "igait-storage",
                "us-east-2".parse().context("Improper region!")?,
                Credentials::default().context("Couldn't unpack credentials! Make sure that you have set AWS credentials in your system environment.")?,
            ).context("Failed to initialize bucket!")?
        })
    }
}

async fn check_dir( app: Arc<Mutex<AppState>>, entry: &DirEntry ) -> Result<()> {
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
    let user_id = dir_name_chunks
        .next()
        .context("Must have valid folder name in format '<id>_<job-id>'!")?;
    let job_id = dir_name_chunks
        .next()
        .context("Must have valid folder name in format '<id>_<job-id>'!")?;

    // Ping the database for the status of the job using 
    //  the user ID and job ID.
    let status = app.lock().await
        .db.get_status( user_id.to_string(), job_id.parse::<usize>().context("File had invalid job ID!")?).await
        .ok_or_else(|| {
            // Purge that directory
            if let Err(e) = remove_dir_all(format!("queue/{}", dir_name)).context("FAILED TO REMOVE") {
                return e;
            }

            anyhow!("\t\tJob didn't exist - Purging files accordingly.")
        })?;

    // If the status is processing or submitting, we don't need to do anything,
    //  the backend is already working on it.
    if status.code == StatusCode::Processing || status.code == StatusCode::Submitting {
        return Ok(());
    }

    // If we're here, we have an unusual status code that we need to handle,
    //  we'll purge the directory and update the status accordingly.
    if status.code == StatusCode::InferenceErr || status.code == StatusCode::SubmissionErr || status.code == StatusCode::Complete {
        print_be("\n----- [ State Update ] -----");
        print_be(&format!("Unusual status code detected, purging accordingly: {:?}", status.code));
    
        // Purge that directory
        remove_dir_all(format!("queue/{}", dir_name)).context(format!("FAILED TO REMOVE 'queue/{}'!", dir_name))?;
    }

    // If it's in the queue, and we're at this state in the code, we
    //  can go ahead and post the request to METIS.
    print_be("\n----- [ State Update ] -----");
    print_be(&format!("Top option (Job {job_id} for '{user_id}') not processing! Firing inference job request..."));
        
    // Update the status of the job to 'Processing'
    app.lock().await.db.update_status(
        user_id.to_string(),
        job_id.parse::<usize>().context("File had invalid job ID!")?,
        Status {
            code: StatusCode::Processing,
            value: String::from("Querying METIS and awaiting response...")
        }
    ).await;
    
    // Query METIS and handle any errors.
    let query_result = query_metis(
        user_id.to_string(), job_id.to_string(),
        std::env::var("AWS_ACCESS_KEY_ID").context("MISSING 'AWS_ACCESS_KEY_ID' in environment!")?,
        std::env::var("AWS_SECRET_ACCESS_KEY").context("Missing 'AWS_SECREt_ACCESS_KEY' in environment!")?,
        std::env::var("IGAIT_ACCESS_KEY").context("Missing 'IGAIT_ACCESS_KEY' in environment!")?
    ).await;
    
    // If the query failed, we'll update the status of the job to reflect that.
    if let Err(reason) = query_result {
        app.lock().await.db.update_status(
            user_id.to_string(),
            job_id.parse::<usize>().context("File had invalid job ID!")?,
            Status {
                code: StatusCode::InferenceErr,
                value: format!("Couldn't query METIS for reason '{reason}'!")
            }
        ).await;

        return Err(anyhow!(reason)).context("Couldn't query METIS!");
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
                print_be(&format!("{e:?}\n\nContinuing as usual..."));
            }
        }
    }
}