use std::{sync::Arc, time::Duration};

use tokio::{fs::DirEntry, sync::Mutex, time::sleep};
use anyhow::{ Result, Context, anyhow };

use crate::{helper::{lib::{copy_file, metis_output_exists, AppState, JobStatus, JobStatusCode, SSHPath}, metis::{query_metis, METIS_HOSTNAME, METIS_OUTPUTS_DIR, METIS_OUTPUT_NAME, METIS_USERNAME}}, print_be};

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

    let pbs_job_id;
    match tokio::fs::read_to_string(&format!("queue/{}/pbs_job_id", dir_name)).await {
        Ok(pbs_job_id_full) => { 
            if metis_output_exists(
                METIS_USERNAME,
                METIS_HOSTNAME, 
                METIS_OUTPUT_NAME,
                &pbs_job_id_full
            ).await
                .context("Couldn't check if the Metis output existed!")?
            {
                pbs_job_id = pbs_job_id_full
                    .split(".")
                    .next()
                    .context("Must at least have a period and some characters in job ID! (Probably unreachable)")?
                    .to_owned();
                print_be!(0, "Found output for '{dir_name}' (PBS Job ID '{pbs_job_id}') :3");
            } else {
                print_be!(0, "[CAN IGNORE] Still awaiting output for '{dir_name}' (PBS Job ID '{pbs_job_id_full}')...");
                return Ok(());
            }
        },
        Err(e) => {
            print_be!(0, "[CAN IGNORE] Couldn't get PBS Job ID on directory '{dir_name}' for reason '{e:?}'");
            return Ok(());
        }
    }

    print_be!(0, "Copying file from Metis home directory to output directory...");
    copy_file(
        "z1994244",
        "metis.niu.edu",
        SSHPath::Remote(&format!("{METIS_OUTPUT_NAME}.o{pbs_job_id}")),
        SSHPath::Remote(
            &format!(
                "{}/{}",
                METIS_OUTPUTS_DIR,
                dir_name
            )
        ),
        false
    ).await
        .context("Couldn't move file from local to Metis!")?;
    print_be!(0, "Copied PBS logfile to output directory successfully!");
    
    // Update the status of the job to 'Processing'
    /*
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
     */

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

        sleep(Duration::from_secs(60)).await;
    }
}