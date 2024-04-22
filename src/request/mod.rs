use serde::{Serialize, Deserialize};
use openssh::{Session, KnownHosts};
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

pub async fn query_metis (
    user_id: String, job_id: String ,
    _aws_access_key_id: String, _aws_secret_access_key: String, _igait_access_key: String
) -> Result<(), String> {
    println!("\n----- [ Querying METIS ] -----");
    let session = Session::connect_mux("igait@metis.niu.edu", KnownHosts::Strict).await
        .map_err(|_| String::from("Couldn't connect to METIS! Are your credentials correct?"))?;
    print_metis("Connected!");

    let run_inference = session
        .command("qsub")
        .arg("-v")
        .arg(
            &format!("USER_ID={user_id},JOB_ID={job_id}")
        )
        .arg("/lstr/sahara/zwlab/jw/igait-ml-backend/pbs/igait.pbs")
        .output().await
        .map_err(|_| String::from("Failed to run openpose command!"))?;
    print_metis(&format!("Output - {}", String::from_utf8(run_inference.stdout).expect("server output was not valid UTF-8")));


    session.close().await
        .expect("Failed to close SSH session - probably fine, continuing.");
    
    Ok(())
}