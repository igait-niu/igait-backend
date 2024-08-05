use crate::print_metis;

use openssh::{Session, KnownHosts};
use anyhow::{ Context, Result };

use super::lib::JobTaskID;

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