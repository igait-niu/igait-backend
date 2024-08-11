use std::{sync::Arc, time::Duration};

use tokio::{fs::DirEntry, sync::Mutex, time::sleep};
use anyhow::{ Result, Context, anyhow };

use crate::{helper::{lib::{AppState, JobStatus, JobStatusCode}, metis::query_metis}, print_be};

/// Checks the directory for a given entry and updates the status of the job accordingly.
/// 
/// # Arguments
/// * `app` - The application state
/// * `entry` - The directory entry to check
/// 
/// # Fails
/// * If the path is invalid Unicode
/// * If the parser finds a malformed file name
/// * If the directory couldn't be removed
/// * If the job didn't exist
/// * If the status couldn't be updated to 'Processing'
/// * If the status couldn't be updated to 'InferenceErr'
/// * If the query to METIS failed
/// 
/// # Returns
/// * A successful result if the directory was checked
/// 
/// # Notes
/// * If the status is 'Processing' or 'Submitting', the function will return early
/// * If the status is 'InferenceErr', 'SubmissionErr', or 'Complete', the directory will be purged
/// * If the status is 'Queue', the status will be updated to 'Processing' and the METIS query will be fired
/// * If the METIS query fails, the status will be updated to 'InferenceErr'
/// * The directory name must be in the format '\<id\>_\<job-id\>'
/// * The directory name must not be '.gitignore'
/// * This function is used in a loop to check the queue directory for new jobs
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
            std::fs::remove_dir_all(format!("queue/{}", dir_name))
                .context("Failed to remove directory!")?;

            Err(anyhow!("\t\tJob didn't exist - Purging files accordingly."))
        })?;

    // If the status is processing or submitting, we don't need to do anything,
    //  the backend is already working on it.
    if status.code == JobStatusCode::Processing || status.code == JobStatusCode::Submitting {
        print_be!(0, "Status is 'Processing' or 'Submitting', skipping...");
        return Ok(());
    }

    // If we're here, we have an unusual status code that we need to handle,
    //  we'll purge the directory and update the status accordingly.
    if status.code == JobStatusCode::InferenceErr || status.code == JobStatusCode::SubmissionErr || status.code == JobStatusCode::Complete {
        print_be!(0, "\n----- [ State Update ] -----");
        let code = status.code;
        print_be!(0, "Unusual status code detected, purging accordingly: {code:#?}");
    
        // Purge that directory
        std::fs::remove_dir_all(format!("queue/{}", dir_name))
            .context(format!("FAILED TO REMOVE 'queue/{}'!", dir_name))?;
    }

    // If it's in the queue, and we're at this state in the code, we
    //  can go ahead and post the request to METIS.
    print_be!(0, "\n----- [ State Update ] -----");
    print_be!(0, "Top option (Job {job_id} for '{uid}') not processing! Firing inference job request...");
        
    // Update the status of the job to 'Processing'
    app.lock().await.db.update_status(
            uid,
            job_id,
            JobStatus {
                code: JobStatusCode::Processing,
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
                JobStatus {
                    code: JobStatusCode::InferenceErr,
                    value: format!("Couldn't query METIS for reason '{reason}'!")
                },
                0
            ).await
            .context("Failed to update status to 'InferenceErr'!")?;

        Err(anyhow!("Couldn't query METIS for reason '{reason}'!"))?
    }

    Ok(())
}

/// The work queue daemon, which checks the queue directory for new jobs.
/// 
/// # Arguments
/// * `app` - The application state
/// 
/// # Fails
/// * If the queue directory couldn't be read
/// * If the queue directory couldn't be iterated over
/// * If the directory couldn't be checked
/// 
/// # Notes
/// * This function never returns, ideally it should be run in a separate thread.
pub async fn work_queue(
    app: Arc<Mutex<AppState>>
) -> () {
    loop {
        sleep(Duration::from_secs(5)).await;

        match tokio::fs::read_dir("queue").await {
            Ok(mut dir) => {
                while let Ok(Some(entry)) = dir.next_entry().await {
                    if let Err(e) = check_dir(app.clone(), &entry).await {
                        print_be!(0, "Failed to process file:\n\n{e:?}\n\nContinuing as usual...");
                    }
                }
            },
            Err(why) => {
                print_be!(0, "Couldn't read from queue directory!");
                print_be!(0, "{why:?}\n\nContinuing as usual...");
                continue;
            }
        }
    }
}