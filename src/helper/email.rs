
use std::time::SystemTime;

use anyhow::{ Context, Result };
use chrono::{ DateTime, Utc };

use crate::print_be;

use super::lib::{Job, JobStatus, JobTaskID};


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