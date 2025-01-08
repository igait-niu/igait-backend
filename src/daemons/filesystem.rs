use std::{sync::Arc, time::Duration};

use tokio::{fs::DirEntry, sync::Mutex, time::sleep};
use anyhow::{ Result, Context, anyhow };
use async_recursion::async_recursion;

use crate::{helper::{lib::{copy_file, delete_logfile, metis_output_exists, AppState, JobStatus, JobStatusCode, SSHPath}, metis::{query_metis, METIS_HOSTNAME, METIS_OUTPUTS_DIR, METIS_OUTPUT_NAME, METIS_USERNAME}}, print_be, print_s3};

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
async fn check_inputs_dir(
    _app:         Arc<Mutex<AppState>>,
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
                print_be!(0, "Found output for '{dir_name}' (PBS Job ID '{pbs_job_id}') :3");
            } else {
                print_be!(0, "[CAN IGNORE] Still awaiting output for '{dir_name}' (PBS Job ID '{pbs_job_id_inner}')...");
                return Ok(());
            }
        },
        Err(e) => {
            print_be!(0, "[CAN IGNORE] Couldn't get PBS Job ID on directory '{dir_name}' for reason '{e:?}'");
            return Ok(());
        }
    }

    print_be!(0, "Copying file from Metis home directory to output directory...");
    let job_id_no_system_postfix = pbs_job_id
        .split(".")
        .next()
        .context("Must at least have a period and some characters in job ID! (Probably unreachable)")?
        .to_owned();
    copy_file(
        "z1994244",
        "metis.niu.edu",
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
    print_be!(0, "Copied PBS logfile to output directory successfully!");

    print_be!(0, "Cleaning logfile from home directory on Metis...");
    delete_logfile( 
        METIS_USERNAME,
        METIS_HOSTNAME, 
        METIS_OUTPUT_NAME,
        &pbs_job_id
    ).await
        .context("Failed to clean up PBS logfile from Metis home directory!")?;
    print_be!(0, "Done!");

    print_be!(0, "Deleting local input folder...");
    tokio::fs::remove_dir_all(&format!("inputs/{dir_name}"))
        .await
        .context("Couldn't remove local input folder!")?;
    print_be!(0, "Successfully deleted local input folder!");

    print_be!(0, "Copying output results from Metis to local...");
    copy_file(
        "z1994244",
        "metis.niu.edu",
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
    print_be!(0, "Successfully copied output from Metis to local!");
    
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

#[async_recursion]
async fn upload_output_dir (
    app: Arc<Mutex<AppState>>,
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

                if &file_name == "json" {
                    print_be!(0, "Skipping JSON upload...");
                    continue;
                }
                    
                if entry.file_type()
                    .await
                    .context("Couldn't get the file type!")?
                    .is_file()
                {
                    let contents = tokio::fs::read(&format!("outputs/{path}/{}", file_name.as_str()))
                        .await
                        .map_err(|e| anyhow!("Couldn't read file `{file_name}`! Error: {e:?}"))?;
                    let size = contents.len();

                    print_s3!(0, "Preparing to upload file `{file_name}` (size {size}, path `{path}`) to S3...");
                    app.lock()
                        .await
                        .bucket
                        .put_object(format!("{user_id}/outputs/{path}/{file_name}"), &contents)
                        .await 
                        .expect("Failed to upload file to S3! Continuing regardless.");
                    print_s3!(0, "Successfully uploaded file `{file_name}` to S3!");
                } else {
                    print_be!(0, "Recursing through sub-directory `{file_name}`");
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
            print_be!(0, "Couldn't read from directory path {path}!");
            print_be!(0, "{why:?}\n\nContinuing as usual...");
        }
    }

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
pub async fn work_outputs(
    app: Arc<Mutex<AppState>>
) -> () {
    match tokio::fs::read_dir("outputs").await {
        Ok(mut dir) => {
            while let Ok(Some(entry)) = dir.next_entry().await {
                let file_name = if let Ok(file_name) = entry
                    .file_name()
                    .into_string()
                {
                    file_name
                } else {
                    let file_name = entry.file_name();
                    println!("[ ERROR ] Output directory `{file_name:?}` is invalidly named!");
                    continue;
                };

                if &file_name == ".gitignore" {
                    continue;
                }

                let user_id = if let Some(user_id) = file_name.split("-").next() {
                    user_id.to_owned()
                } else {
                    println!("[ ERROR ] Output directory `{file_name}` is invalidly named!");
                    continue;
                };
                
                print_s3!(0, "Uploading directory `{file_name}` to S3...");
                if let Err(e) = upload_output_dir(
                    app.clone(),
                    user_id,
                    file_name.to_string()
                ).await {
                    println!("[ WARN ] Encountered error uploading outputs directory `{file_name}`! Error: {e:?}");
                }
                print_s3!(0, "Successfully uploaded directory `{file_name}` to S3!");
                
                print_be!(0, "Deleting local output folder...");
                if let Err(e) = tokio::fs::remove_dir_all(&format!("outputs/{file_name}")).await {
                    println!("[ ERROR ] Couldn't remove local input folder! Error: {e:?}");
                    continue;
                }
                print_be!(0, "Successfully deleted local output folder!");
            }
        },
        Err(e) => {
            print_be!(0, "Encountered error trying to work output directory! Error: {e:?}");
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
    app: Arc<Mutex<AppState>>
) {
    loop {
        print_be!(0, "Scanning inputs...");

        match tokio::fs::read_dir("inputs").await {
            Ok(mut dir) => {
                while let Ok(Some(entry)) = dir.next_entry().await {
                    if let Err(e) = check_inputs_dir(app.clone(), &entry).await {
                        print_be!(0, "Failed to process file:\n\n{e:?}\n\nContinuing as usual...");
                    }
                }
            },
            Err(why) => {
                print_be!(0, "Couldn't read from inputs directory!");
                print_be!(0, "{why:?}\n\nContinuing as usual...");
                continue;
            }
        }

        print_be!(0, "Scanning outputs...");
        work_outputs(app.clone()).await;

        sleep(Duration::from_secs(15)).await;
    }
}