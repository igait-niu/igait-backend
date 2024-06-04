use serde::{Serialize, Deserialize};
use openssh::{Session, KnownHosts};
use anyhow::{ Context, Result };
use crate::print::print_metis;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum StatusCode {
    Submitting,
    SubmissionErr,
    Queue,
    Processing,
    InferenceErr,
    Complete
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub id: usize,
    pub status: StatusCode
}

pub async fn query_metis ( uid: &str, job_id: usize ) -> Result<()> {
    // Attempt to connect to METIS
    print_metis("\n----- [ Querying METIS ] -----");
    let session = Session::connect_mux("igait@metis.niu.edu", KnownHosts::Strict)
        .await
        .context("Couldn't connect to METIS! Are your credentials correct?")?;
    print_metis("Connected!");

    // Run the inference job
    let run_inference = session
        .command("qsub")
        .arg("-v")
        .arg(&format!("USER_ID={uid},JOB_ID={job_id}"))
        .arg("/lstr/sahara/zwlab/jw/igait-ml-backend/pbs/igait.pbs")
        .output().await
        .context("Failed to run openpose command!")?;
    print_metis(&format!("Output - {}", String::from_utf8(run_inference.stdout).context("Server output was not valid UTF-8")?));

    // Close the SSH session
    session.close().await
        .context("Failed to close SSH session - probably fine.")?;

    // Return as successful
    print_metis("Successfully queried METIS!");
    Ok(())
}