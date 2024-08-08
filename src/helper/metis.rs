use crate::print_metis;

use openssh::{Session, KnownHosts};
use anyhow::{ Context, Result };

use super::lib::JobTaskID;

/// Queries METIS for the specified job.
/// 
/// # Arguments
/// * `uid` - The user ID of the job
/// * `job_id` - The job ID of the job
/// * `task_number` - The task number of the job
/// 
/// # Fails
/// * If the SSH session fails to connect
/// * If the SSH command fails to run
/// * If the server output is not valid UTF-8
/// * If the SSH session fails to close
/// * If the METIS query fails
/// 
/// # Returns
/// * A successful result if the METIS query was successful
/// 
/// # Notes
/// It's important to understand that this is essentially automating the process of submitting a job to METIS via SSH.
/// 
/// The way this works is by calling a Python script which takes in the user ID and job ID as environment variables.
/// From there, the script first pulls the files from AWS, then runs the OpenPose inference on the files.
/// 
/// After Metis generates a result, the script then uploads the results back to AWS.
/// 
/// Metis then makes a POST request to the iGait API with the results via the [`completion`](crate::routes::completion) endpoint.
pub async fn query_metis (
    uid:         &str,
    job_id:      usize,
    task_number: JobTaskID
) -> Result<()> {
    // Attempt to connect to METIS
    print_metis!(task_number, "\n----- [ Querying METIS ] -----");
    let session = Session::connect_mux("igait@metis.niu.edu", KnownHosts::Strict)
        .await
        .context("Couldn't connect to METIS! Are your credentials correct?")?;
    print_metis!(task_number, "Connected!");

    // Run the inference job
    let run_inference = session
        .command("qsub")
        .arg("-v")
        .arg(&format!("USER_ID={uid},JOB_ID={job_id}"))
        .arg("/lstr/sahara/zwlab/jw/igait-ml-backend/pbs/igait.pbs")
        .output().await
        .context("Failed to run openpose command!")?;
    
    let output = String::from_utf8(run_inference.stdout.clone()).context("Server output was not valid UTF-8")?;
    print_metis!(task_number, "Output - {output}");

    // Close the SSH session
    session.close().await
        .context("Failed to close SSH session - probably fine.")?;

    // Return as successful
    print_metis!(task_number, "Successfully queried METIS!");
    Ok(())
}