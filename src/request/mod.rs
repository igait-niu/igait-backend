use serde::{Serialize, Deserialize};
use openssh::{Session, KnownHosts};

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
    aws_access_key_id: String, aws_secret_access_key: String, igait_access_key: String
) -> Result<(), String> {
    let session = Session::connect_mux("z1994244@metis.niu.edu", KnownHosts::Strict).await
        .map_err(|_| String::from("Couldn't connect to METIS! Are your credentials correct?"))?;


    let export_keys = session
        .command(".")
        .arg("/lstr/sahara/zwlab/jw/igait-ml-backend/keys.sh")
        .output().await
        .map_err(|_| String::from("Failed to run keys command!"))?;
    println!("Keys output - {}", String::from_utf8(export_keys.stdout).expect("server output was not valid UTF-8"));

    /*
        let ls = session
            .command("ls")
            .output().await
            .map_err(|_| String::from("Failed to run keys command!"))?;
        println!("ls- {}", String::from_utf8(ls.stdout).expect("server output was not valid UTF-8"));
    */

    // /lstr/sahara/zwlab/jw/igait-ml-backend/.venv/bin/python /lstr/sahara/zwlab/jw/igait-ml-backend/main.py curlplaceholder 14
    let run_inference = session
        .raw_command("/lstr/sahara/zwlab/jw/igait-ml-backend/.venv/bin/python")
        .raw_args(vec!("/lstr/sahara/zwlab/jw/igait-ml-backend/main.py", 
            &user_id, &job_id,
            &aws_access_key_id, &aws_secret_access_key, &igait_access_key
        ))
        .status().await
        .map_err(|err| format!("Err: {err:?}"))?;
    
    println!("{run_inference}");
    //println!("cd - {}", String::from_utf8(run_inference.stdout).expect("server output was not valid UTF-8"));



    session.close().await
        .expect("Failed to close SSH session - probably fine, continuing.");
    
    Ok(())
}