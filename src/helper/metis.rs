#![doc = include_str!("../docs/metis.md")]

use openssh::{KnownHosts, Session};
use anyhow::{ Result, Context, bail, anyhow };
use tokio::process::Command;

pub const METIS_USERNAME:    &'static str = "z1994244";
pub const METIS_HOSTNAME:    &'static str = "metis.niu.edu";
pub const METIS_PBS_PATH:    &'static str = "/lstr/sahara/zwlab/data/scripts/test.pbs";
pub const METIS_OUTPUT_NAME: &'static str = "igait_prod";
pub const METIS_INPUTS_DIR:  &'static str = "/lstr/sahara/zwlab/data/inputs";
pub const METIS_OUTPUTS_DIR: &'static str = "/lstr/sahara/zwlab/data/outputs";
pub const METIS_DATA_DIR:    &'static str = "/lstr/sahara/zwlab/data";

pub enum SSHPath<'a> {
    Local  (&'a str),
    Remote (&'a str)
}
pub async fn copy_file<'a> (
    username:         &str,
    hostname:         &str,

    source:           SSHPath<'a>,
    destination:      SSHPath<'a>,
    directory:        bool
) -> Result<String> {
    let mut command = Command::new("scp");

    if directory {
        command.arg("-r");
    }

    match source {
        SSHPath::Remote(remote_file_path) => {
            match destination {
                SSHPath::Local(local_file_path) => {
                    command
                        .arg(format!("{username}@{hostname}:{}", remote_file_path ))
                        .arg(local_file_path);
                },
                SSHPath::Remote(new_remote_file_path) => {
                    command
                        .arg(format!("{username}@{hostname}:{}", remote_file_path ))
                        .arg(format!("{username}@{hostname}:{}", new_remote_file_path ));
                }
            }
        },
        SSHPath::Local(local_file_path) => {
            if let SSHPath::Remote(remote_file_path) = destination {
                command
                    .arg(local_file_path)
                    .arg(format!("{username}@{hostname}:{}", remote_file_path));
            } else {
                bail!("Must have differing SSHPath types!");
            }
        }
    }

    let output = command.output()
        .await
        .context("Failed to execute `scp` command!")?;

    let stdout: String = String::from_utf8 ( output.stdout )
        .context("Standard output contained invalid UTF-8!")?;
    let stderr: String = String::from_utf8 ( output.stderr )
        .context("Standard error contained invalid UTF-8!")?;

    if !stderr.is_empty() {
        bail!("Got error output: {stderr}");
    }

    Ok(stdout)
}

pub type PBSId = String;
pub async fn metis_qsub (
    username: &str,
    hostname: &str,

    pbs_path: &str,
    args: Vec<&str>
) -> Result<PBSId> {
    // Attempt to connect to METIS
    let session = Session::connect_mux(&format!("{username}@{hostname}"), KnownHosts::Strict)
        .await
        .map_err(|e| anyhow::anyhow!("Error starting Metis connection! See below:\n{:#?}", e))?;

    // Add our args
    let mut command = session
        .command("qsub");
    for arg in &args {
        command.arg(arg);
    }

    // Run the job
    let output = command
        .arg(pbs_path)
        .output().await
        .context("Failed to run openpose command!")?;

    // Extract the output from stdout
    let stdout = String::from_utf8(output.stdout)
        .context("Server `stdout` was not valid UTF-8")?;
    let stderr = String::from_utf8(output.stderr)
        .context("Server `stderr` was not valid UTF-8")?;

    // Close the SSH session
    session.close().await
        .context("Failed to close SSH session - probably fine.")?;

    // Treat any error output as fatal
    if !stderr.is_empty() {
        bail!("Server had `stderr`: {stderr}");
    }

    // Return as successful
    Ok(stdout.trim().into())
}

pub async fn metis_output_exists (
    username:  &str,
    hostname:  &str,

    pbs_job_name: &str,
    pbs_job_id:   &str
) -> Result<bool> {
    // Extract the job number since there's additional information in the job ID
    let pbs_job_number = pbs_job_id
        .split('.')
        .next()
        .ok_or(anyhow!("Missing job number! Ensure the Job ID is in the form <n>.cm!"))?;

    // Attempt to connect to METIS
    let session = Session::connect_mux(&format!("{username}@{hostname}"), KnownHosts::Strict)
        .await
        .map_err(|e| anyhow::anyhow!("Error starting Metis connection! See below:\n{:#?}", e))?;

    // Add our path and run the command
    let output = session
        .command("ls")
        .arg(format!("{pbs_job_name}.o{pbs_job_number}"))
        .output().await
        .context("Failed to run openpose command!")?;

    // Extract the output from stdout
    let _stdout = String::from_utf8(output.stdout)
        .context("Server `stdout` was not valid UTF-8")?;
    let stderr = String::from_utf8(output.stderr)
        .context("Server `stderr` was not valid UTF-8")?;

    // Close the SSH session
    session.close().await
        .context("Failed to close SSH session - probably fine.")?;

    // Return as successful
    Ok(stderr.is_empty())
}

pub async fn delete_logfile (
    username:  &str,
    hostname:  &str,

    pbs_job_name: &str,
    pbs_job_id:   &str
) -> Result<()> {
    // Extract the job number since there's additional information in the job ID
    let pbs_job_number = pbs_job_id
        .split('.')
        .next()
        .ok_or(anyhow!("Missing job number! Ensure the Job ID is in the form <n>.cm!"))?;

    // Attempt to connect to METIS
    let session = Session::connect_mux(&format!("{username}@{hostname}"), KnownHosts::Strict)
        .await
        .map_err(|e| anyhow::anyhow!("Error starting Metis connection! See below:\n{:#?}", e))?;

    // Add our path and run the command
    let output = session
        .command("rm")
        .arg(format!("{pbs_job_name}.o{pbs_job_number}"))
        .output().await
        .context("Failed to run openpose command!")?;

    // Extract the output from stdout
    let _stdout = String::from_utf8(output.stdout)
        .context("Server `stdout` was not valid UTF-8")?;
    let stderr = String::from_utf8(output.stderr)
        .context("Server `stderr` was not valid UTF-8")?;

    // Close the SSH session
    session.close().await
        .context("Failed to close SSH session - probably fine.")?;

    if !stderr.is_empty() {
        bail!("Likely failed to delete logfile - full error: {stderr}");
    }

    // Return as successful
    Ok(())
}

pub async fn delete_output_folder (
    username:  &str,
    hostname:  &str,

    uid: &str,
    job_id:   &str
) -> Result<()> {
    // Attempt to connect to METIS
    let session = Session::connect_mux(&format!("{username}@{hostname}"), KnownHosts::Strict)
        .await
        .map_err(|e| anyhow::anyhow!("Error starting Metis connection! See below:\n{:#?}", e))?;

    // Add our path and run the command
    let output = session
        .command("rm")
        .args(vec!("-rf", &format!("{METIS_OUTPUTS_DIR}/{uid};{job_id}")))
        .output().await
        .context("Failed to run openpose command!")?;

    // Extract the output from stdout
    let _stdout = String::from_utf8(output.stdout)
        .context("Server `stdout` was not valid UTF-8")?;
    let stderr = String::from_utf8(output.stderr)
        .context("Server `stderr` was not valid UTF-8")?;

    // Close the SSH session
    session.close().await
        .context("Failed to close SSH session - probably fine.")?;

    if !stderr.is_empty() {
        bail!("Likely failed to delete logfile - full error: {stderr}");
    }

    // Return as successful
    Ok(())
}
