use openssh::{KnownHosts, Session};
use anyhow::{ Result, Context, bail };
use tokio::process::Command;

pub const METIS_USERNAME:    &str = "igait";
pub const METIS_HOSTNAME:    &str = "metis.niu.edu";
pub const METIS_PBS_PATH:    &str = "/lstr/sahara/zwlab/jw/igait-pipeline/igait-pipeline/pipeline.pbs";
pub const METIS_INPUTS_DIR:  &str = "/lstr/sahara/zwlab/data/inputs";

/// Enum representing either a local or remote path, for use with the `copy_file` function
pub enum SSHPath<'a> {
    Local  (&'a str),
    Remote (&'a str)
}

/// A function that allows the user to copy files using the SCP command between one or two
/// machines.
///
/// # Arguments
/// * `username`: The username of the remote machine
/// * `hostname`: The hostname of the remote machine
/// * `source`: The path of the source file you wish to copy
/// * `destination`: The path of the destination file you wish to copy to
/// * `directory`: Whether or not the source file is actually a directory
///
/// # Returns
/// * The `stdout` from the `scp` command if it was successful
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

/// Submits a PBS job to Metis via qsub
///
/// # Arguments
/// * `username`: The username on Metis
/// * `hostname`: The hostname of Metis
/// * `pbs_path`: The path to the PBS script on Metis
/// * `args`: Additional arguments to pass to qsub (e.g., "-v ID=value")
///
/// # Returns
/// * The PBS job ID returned by qsub
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
