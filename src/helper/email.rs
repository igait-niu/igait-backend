
use std::time::SystemTime;

use anyhow::{ Context, Result };
use chrono::{ DateTime, Utc };

use crate::print_be;

use super::lib::{Job, JobStatus, JobTaskID};

/// Sends an email to the specified address with the specified subject and body.
/// 
/// # Arguments
/// * `to` - The email address to send the email to
/// * `subject` - The subject of the email
/// * `body` - The body of the email
/// 
/// # Fails
/// * If the `IGAIT_ACCESS_KEY` environment variable is missing
/// * If the form fails to send to the Cloudflare Worker
/// 
/// # Returns
/// * A successful result if the email was sent
/// 
/// # Notes
/// * The email is sent to the Cloudflare Worker at `https://email-service.igaitniu.workers.dev/`
/// # The environment variable `IGAIT_ACCESS_KEY` is used to authenticate the request and must be set
pub fn send_email (
    to:      &str,
    subject: &str,
    body:    &str,
    task_number: JobTaskID
) -> Result<()> {
    print_be!(task_number, "Sending email to '{to}'...");

    // Post the form to the Cloudflare Worker
    ureq::post("https://email-service.igaitniu.workers.dev/")
        .send_form(&[
            ( "API_KEY", &std::env::var("IGAIT_ACCESS_KEY").context("MISSING IGAIT_ACCESS_KEY!")? ),
            ( "to",      to      ),
            ( "subject", subject ),
            ( "body",    body    )
        ])
        .context("Failed to send form to the Cloudllare Worker")?;
    print_be!(task_number, "Successfully sent email to '{to}'!");

    Ok(())
}

/// A wrapper around `send_email` that sends a success email to the recipient.
/// 
/// # Arguments
/// * `recipient_email_address` - The email address to send the email to
/// * `status` - The status of the job
/// * `dt_timestamp_utc` - The timestamp of the job
/// * `job` - The job that was submitted
/// * `front_keyframed_url` - The URL to the front keyframed video
/// * `side_keyframed_url` - The URL to the side keyframed video
/// * `uid` - The user ID of the job
/// * `job_id` - The job ID of the job
/// * `task_number` - The task number of the job
/// 
/// # Fails
/// * If the email fails to send
/// 
/// # Returns
/// * A successful result if the email was sent
/// 
/// # Notes
/// * Any changes to the email logic should be made to the `send_email` function first
pub async fn send_success_email (
    recipient_email_address: &str,
    status:                  &JobStatus,
    dt_timestamp_utc:        &DateTime<Utc>,
    job:                     &Job,
    front_keyframed_url:     &str,
    side_keyframed_url:      &str,
    uid:                     &str,
    job_id:                  usize,
    task_number:             JobTaskID
) -> Result<()> {
    // Build the email
    let subject = format!("Your recent submission to iGait App has completed!");
    let body = format!("We deteremined a likelyhood score of {} for your submission on {} (UTC)!<br><br>Submission information:<br>Age: {}<br>Ethnicity: {}<br>Sex: {}<br>Height: {}<br>Weight: {}<br><br>Front Video: {}<br>Side Video: {}<br>These videos will remain downloadable for 7 days from the date of this email. If they expire, contact GaitStudy@niu.edu to have new files issued. If you recieve an error message viewing these videos, please use a different browser such as Chrome.<br><br>User ID: {}<br>Job ID: {}", 
        status.value,
        dt_timestamp_utc.format("%m/%d/%Y at %H:%M"),

        job.age,
        job.ethnicity,
        job.sex,
        job.height,
        job.weight,

        front_keyframed_url,
        side_keyframed_url,

        uid,
        job_id
    );

    // Send the email
    send_email( recipient_email_address, &subject, &body, task_number )
}

/// A wrapper around `send_email` that sends a failure email to the recipient.
/// 
/// # Arguments
/// * `recipient_email_address` - The email address to send the email to
/// * `status` - The status of the job
/// * `dt_timestamp_utc` - The timestamp of the job
/// * `uid` - The user ID of the job
/// * `job_id` - The job ID of the job
/// * `task_number` - The task number of the job
/// 
/// # Fails
/// * If the email fails to send
/// 
/// # Returns
/// * A successful result if the email was sent
/// 
/// # Notes
/// * Any changes to the email logic should be made to the `send_email` function first
pub async fn send_failure_email (
    recipient_email_address: &str,
    status:                  &JobStatus,
    dt_timestamp_utc:        &DateTime<Utc>,
    uid:                     &str,
    job_id:                  usize,
    task_number:             JobTaskID
) -> Result<()> {
    // Build the email
    let subject = format!("Your recent submission to iGait App failed!");
    let body = format!("Something went wrong with your submission on {}!<br><br>Error Type: '{:?}'<br>Error Reason: '{}'<br><br>User ID: {}<br>Job ID: {}<br><br><br>Please contact support:<br>GaitStudy@niu.edu",
        dt_timestamp_utc.format("%m/%d/%Y at %H:%M"),

        status.code, status.value, 
        uid,
        job_id
    );

    // Send the email
    send_email( recipient_email_address, &subject, &body, task_number )
}

/// A wrapper around `send_email` that sends a welcome email to the recipient.
/// 
/// # Arguments
/// * `job` - The job that was submitted
/// * `uid` - The user ID of the job
/// * `job_id` - The job ID of the job
/// * `task_number` - The task number of the job
/// 
/// # Fails
/// * If the email fails to send
/// 
/// # Returns
/// * A successful result if the email was sent
/// 
/// # Notes
/// * Any changes to the email logic should be made to the `send_email` function first
pub async fn send_welcome_email (
    job:         &Job,
    uid:         &str,
    job_id:      usize,
    task_number: JobTaskID
) -> Result<()> {
    // Build the email
    let dt_now_utc: DateTime<Utc> = SystemTime::now().into();
    let subject = format!("Welcome to iGait!");
    let body = format!("Your job submission on {} (UTC) has been uploaded successfully! Please give us 1-2 days to complete analysis.<br><br>Submission information:<br>Age: {}<br>Ethnicity: {}<br>Sex: {}<br>Height: {}<br>Weight: {}<br><br>User ID: {}<br>Job ID: {}", 
        dt_now_utc.format("%m/%d/%Y at %H:%M"),

        job.age,
        job.ethnicity,
        job.sex,
        job.height,
        job.weight,

        uid,
        job_id
    );

    // Send the email
    send_email( &job.email, &subject, &body, task_number )
}