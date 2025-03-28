use std::{sync::Arc, time::Duration};

use chrono::{DateTime, Utc};
use tokio::{fs::DirEntry, time::sleep};
use anyhow::{ Result, Context, anyhow };
use async_recursion::async_recursion;
use tracing::{info, warn, error};

use crate::{helper::{email::{send_failure_email, send_success_email}, lib::{AppState, Job, JobStatus, JobStatusCode}, metis::{copy_file, delete_logfile, delete_output_folder, metis_output_exists, SSHPath, METIS_HOSTNAME, METIS_OUTPUTS_DIR, METIS_OUTPUT_NAME, METIS_USERNAME}}, print_be, print_metis, print_s3, ASD_CLASSIFICATION_THRESHOLD, DISABLE_RESULT_EMAIL};

/// Checks the a directory entry from the `inputs` folder, and preforms the following:
///
/// - 1.) Waits for a successful output
/// - 2.) Copies logfile to output directory (on Metis)
/// - 3.) Deletes the original logfile (on Metis)
/// - 4.) Deletes the local `inputs` folder
/// - 5.) Copies the output folder from Metis to the local `outputs` folder
/// 
/// # Arguments
/// * `entry` - The directory entry to check
/// 
/// # Returns
/// * A successful result if the directory was checked
#[tracing::instrument]
async fn check_inputs_dir(
    entry: &DirEntry
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

    let pbs_job_id: String;
    match tokio::fs::read_to_string(&format!("inputs/{}/pbs_job_id", dir_name)).await {
        Ok(pbs_job_id_inner) => { 
            if metis_output_exists(
                METIS_USERNAME,
                METIS_HOSTNAME, 
                METIS_OUTPUT_NAME,
                &pbs_job_id_inner
            ).await
                .context("Couldn't check if the Metis output existed!")?
            {
                pbs_job_id = pbs_job_id_inner;
                info!("Found output for '{dir_name}' (PBS Job ID '{pbs_job_id}') :3");
            } else {
                info!("[CAN IGNORE] Still awaiting output for '{dir_name}' (PBS Job ID '{pbs_job_id_inner}')...");
                return Ok(());
            }
        },
        Err(e) => {
            warn!("[CAN IGNORE] Couldn't get PBS Job ID on directory '{dir_name}' for reason '{e:?}'");
            return Ok(());
        }
    }

    info!("Copying file from Metis home directory to output directory...");
    let job_id_no_system_postfix = pbs_job_id
        .split(".")
        .next()
        .context("Must at least have a period and some characters in job ID! (Probably unreachable)")?
        .to_owned();
    copy_file(
        METIS_USERNAME,
        METIS_HOSTNAME,
        SSHPath::Remote(&format!("{METIS_OUTPUT_NAME}.o{job_id_no_system_postfix}")),
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
    info!("Copied PBS logfile to output directory successfully!");

    info!("Cleaning logfile from home directory on Metis...");
    delete_logfile( 
        METIS_USERNAME,
        METIS_HOSTNAME, 
        METIS_OUTPUT_NAME,
        &pbs_job_id
    ).await
        .context("Failed to clean up PBS logfile from Metis home directory!")?;
    info!("Done!");

    info!("Deleting local input folder...");
    tokio::fs::remove_dir_all(&format!("inputs/{dir_name}"))
        .await
        .context("Couldn't remove local input folder!")?;
    info!("Successfully deleted local input folder!");
    
    info!("Copying output results from Metis to local...");
    copy_file(
        METIS_USERNAME,
        METIS_HOSTNAME,
        SSHPath::Remote(
            &format!(
                "{}/{}",
                METIS_OUTPUTS_DIR,
                dir_name
            )),
        SSHPath::Local("outputs"),
        true
    ).await
        .context("Couldn't move the outputs from Metis to local outputs directory!")?;
    info!("Successfully copied output from Metis to local!");

    Ok(())
}

/// Recursively uploads a given folder to AWS.
///
/// This function will recursive if it encounters a sub-directory, otherwise,
/// it will simply upload directly to AWS based on the current key path that
/// has been built thus far.
///
/// # Arguments
/// - `app`: Handle to the general app state
/// - `user_id`: The user ID the files should be uploaded to on AWS
/// - `path`: The path that the function should base upon (built through recursion)
#[async_recursion]
async fn upload_output_dir (
    app: Arc<AppState>,
    user_id: String,
    path: String
) -> Result<()> {
    match tokio::fs::read_dir(&format!("outputs/{path}")).await {
        Ok(mut dir) => {
            while let Ok(Some(entry)) = dir.next_entry().await {
                let file_name = entry
                    .file_name()
                    .into_string()
                    .map_err(|e| anyhow!("{e:?}"))?;
                    
                if entry.file_type()
                    .await
                    .context("Couldn't get the file type!")?
                    .is_file()
                {
                    let contents = tokio::fs::read(&format!("outputs/{path}/{}", file_name.as_str()))
                        .await
                        .map_err(|e| anyhow!("Couldn't read file `{file_name}`! Error: {e:?}"))?;
                    let size = contents.len();

                    info!("Preparing to upload file `{file_name}` (size {size}, path `{path}`) to S3...");
                    app.bucket
                        .lock().await
                        .put_object(format!("{user_id}/outputs/{path}/{file_name}"), &contents)
                        .await 
                        .expect("Failed to upload file to S3! Continuing regardless.");
                    info!("Successfully uploaded file `{file_name}` to S3!");
                } else {
                    info!("Recursing through sub-directory `{file_name}`");
                    upload_output_dir (
                        app.clone(),
                        user_id.clone(),
                        path.clone() + "/" + file_name.as_str() 
                    ).await
                        .map_err(|e| anyhow!("Failed to recurse `{file_name}` {e:?}"))?
                }
            }
        },
        Err(why) => {
            error!("Couldn't read from directory path {path}!");
            error!("{why:?}\n\nContinuing as usual...");
        }
    }

    Ok(())
}

/// Similar to the input helper, this processes one DirEntry from the `outputs` directory.
///
/// This function does the following:
/// - 1.) Uploads the folder to S3
/// - 2.) Checks if there was output
/// - 3.) Sends an email depending on whether or not there was a successful result
/// - 4.) Deletes the entry from the local `outputs` folder
/// - 5.) Deletes the entry from the `outputs` folder on Metis
///
/// # Arguments
/// * `app`: A handle to the app state
/// * `entry`: The entry to check from the `outputs` directory
#[tracing::instrument]
async fn work_output_helper (
    app: &Arc<AppState>,
    entry: DirEntry
) -> Result<()> {
    let file_name = entry
        .file_name()
        .into_string()
        .map_err(|_| anyhow!("Output directory is invalidly named!"))?;

    if &file_name == ".gitignore" {
        return Ok(());
    }

    let user_id = file_name.split(';')
        .next()
        .context("[ ERROR ] Output directory `{file_name}` is invalidly named!")?
        .to_owned();
    
    info!("Uploading directory `{file_name}` to S3...");
    upload_output_dir(
        app.clone(),
        user_id,
        file_name.to_string()
    ).await
        .map_err(|e| anyhow!("[ WARN ] Encountered error uploading outputs directory `{file_name}`! Error: {e:?}"))?;
    info!("Successfully uploaded directory `{file_name}` to S3!");

    // Get the user and job IDs
    let uid = file_name.split(';')
        .next()
        .context("Directory name was missing user ID!")?;
    let job_id = file_name.split(';')
        .nth(1)
        .context("Directory name was missing user ID!")?
        .parse::<usize>()
        .context("Job ID was not a valid `usize`!")?;

    // Grab the job it references
    let job: Job = app
        .db
        .lock().await
        .get_job(
            uid,
            job_id
        ).await
        .context("The job targeted by the completion request doesn't exist!")?; 

    // Extract the email address and timestamp
    let recipient_email_address = job.email.clone();
    let dt_timestamp_utc: DateTime<Utc> = job.timestamp.into();
    let cst = dt_timestamp_utc.with_timezone(&chrono_tz::US::Central);

    info!("Checking whether `final_score` file exists!");
    if tokio::fs::try_exists(&format!("outputs/{file_name}/final_score")).await
        .map_err(|e| anyhow!("Encountered error trying to find `final_score` file! Error: {e:?}"))?
    {
        let score = tokio::fs::read_to_string(
            &format!("outputs/{file_name}/final_score"))
            .await
            .context("Couldn't read `final_score` file. (likely unreachable)")?
            .parse::<f32>()
            .context("Final score was not a valid `f32`!")?;

        let classification = if score > ASD_CLASSIFICATION_THRESHOLD {
            "ASD"
        } else {
            "NO ASD"
        };

        let status = JobStatus {
            code: JobStatusCode::Complete,
            value: String::from(classification)
        };

        app.db
            .lock().await
            .update_status(
                uid,
                job_id,
                status.clone()
            ).await
            .context("Failed to update status to 'Processing'!")?;
            
        // Send the success email
        if !DISABLE_RESULT_EMAIL {
            send_success_email(
                app.clone(),
                &recipient_email_address,
                &status,
                &cst,
                &job,
                uid,
                job_id,
                0
            ).await.context("Failed to send success email!")?;
        }
    } else {
        let status = JobStatus {
            code: JobStatusCode::InferenceErr,
            value: String::from("There was an error on our end! Please contact support via the instructions in the email containing these results.")
        };

        app.db
        .lock().await
            .update_status(
                uid,
                job_id,
                status.clone()
            ).await
            .context("Failed to update status to 'Processing'!")?;

        // Send the failure email
        send_failure_email(
            app.clone(),
            &recipient_email_address,
            &status,
            &cst,
            uid,
            job_id
        ).await.context("Failed to send success email!")?;
    }
    
    info!("Deleting local output folder...");
    if let Err(e) = tokio::fs::remove_dir_all(&format!("outputs/{file_name}")).await {
        error!("[ ERROR ] Couldn't remove local input folder! Error: {e:?}");
    }
    info!("Successfully deleted local output folder!");

    info!("Deleting output folder off Metis...");
    delete_output_folder(
        METIS_USERNAME,
        METIS_HOSTNAME,

        uid,
        &job_id.to_string()
    ).await
        .context("Failed to delete the output folder off Metis!")?;
    info!("Successfully deleted output folder off Metis!");
    
    Ok(())
}

/// The work inputs daemon, which checks the inputs directory for new jobs.
/// 
/// # Arguments
/// * `app` - The application state
/// 
/// # Fails
/// * If the inputs directory couldn't be read
/// * If the inputs directory couldn't be iterated over
/// * If the directory couldn't be checked
/// 
/// # Notes
/// * This function never returns, ideally it should be run in a separate thread.
#[tracing::instrument]
pub async fn work_outputs(
    app: Arc<AppState>
) {
    match tokio::fs::read_dir("outputs").await {
        Ok(mut dir) => {
            while let Ok(Some(entry)) = dir.next_entry().await {
                if let Err(e) = work_output_helper(&app, entry).await {
                    error!("Encountered error in output worker! Error: {e:?}");
                }
            }
        },
        Err(e) => {
            error!("Encountered error trying to work output directory! Error: {e:?}");
        }
    }
}

/// The work inputs daemon, which checks the inputs directory for new jobs.
/// 
/// # Arguments
/// * `app` - The application state
/// 
/// # Fails
/// * If the inputs directory couldn't be read
/// * If the inputs directory couldn't be iterated over
/// * If the directory couldn't be checked
/// 
/// # Notes
/// * This function never returns, ideally it should be run in a separate thread.
pub async fn work_inputs(
    app: Arc<AppState>
) {
    loop {
        info!("Scanning inputs...");

        match tokio::fs::read_dir("inputs").await {
            Ok(mut dir) => {
                while let Ok(Some(entry)) = dir.next_entry().await {
                    if let Err(e) = check_inputs_dir(&entry).await {
                        error!("Failed to process file:\n\n{e:?}\n\nContinuing as usual...");
                    }
                }
            },
            Err(why) => {
                error!("Couldn't read from inputs directory!");
                error!("{why:?}\n\nContinuing as usual...");
                continue;
            }
        }

        info!("Scanning outputs...");
        work_outputs(app.clone()).await;

        sleep(Duration::from_secs(15)).await;
    }
}
