use std::{sync::Arc, time::SystemTime};

use anyhow::{ Result, Context, anyhow };
use axum::extract::{Multipart, State};
use chrono::{DateTime, Utc};
use tokio::sync::Mutex;

use crate::{helper::{email::send_email, lib::{AppError, AppState, Job, JobStatusCode, JobTaskID}}, print_be, print_s3};

struct HistoricalArguments {
    uid: String
}
async fn unpack_historical_arguments(
    mut multipart: Multipart,
    task_number:   JobTaskID
) -> Result<HistoricalArguments> {
    // Unwrap all fields, which, in this case,
    //  is just the user ID.
    let mut uid_option: Option<String> = None;
    while let Some(field) = multipart
        .next_field().await
        .context("Bad request! Is it possible you submitted a file over the size limit?")?
    {
        let name = field.name();
        print_be!(task_number, "Field Incoming: {name:#?}");
        match field.name() {
            Some("user_id") => {
                uid_option = Some(
                        field
                            .text().await
                            .context("Field 'user_id' wasn't readable as text!")?
                            .to_string());
            },
            _ => {
                print_be!(task_number, "Which had an unknown/no field name...");
            }
        }
    }
    let uid = uid_option.ok_or(anyhow!("Missing 'user_id' in request!"))?;

    Ok(HistoricalArguments {
        uid
    })
}
pub async fn historical_entrypoint ( 
    State(app): State<Arc<Mutex<AppState>>>,
    multipart: Multipart
) -> Result<&'static str, AppError> {
    // Allocate a new task number
    app.lock().await
        .task_number += 1;
    let task_number = app.lock().await.task_number;

    print_be!(task_number, "\n----- [ Recieved historical submissions request ] -----");

    // Unpack the arguments
    print_be!(task_number, "Unpacking arguments...");
    let arguments = unpack_historical_arguments(multipart, task_number).await
        .context("Failed to unpack historical arguments!")?;

    // Get all jobs
    let jobs = app.lock().await
        .db
        .get_all_jobs(
            &arguments.uid,
            task_number
        )
        .await
        .context("Failed to get jobs!")?;

    // Generate the body
    let mut email_body = concat!(
        "<h1>Thank you for contacting iGait!</h1>",
        "You recently requested a complete history of your submissions. Located below can be found, in chronological order, all past submissions.<br>"
        ).to_string();
    for job in jobs.iter() {
        // Add a condensed version of the job to the shortened body
        let dt_timestamp_utc: DateTime<Utc> = job.timestamp.into();
        email_body.push_str(&format!(
            "<h2>{}</h2>",
            dt_timestamp_utc.format("%Y-%m-%d %H:%M:%S UTC")
        ));
        match job.status.code {
            JobStatusCode::Complete => {
                email_body.push_str(
                    &format!(
                        "- Status: Complete<br>- Confidence: {:.2}%", 
                        job.status.value
                            .parse::<f64>()
                            .context("Failed to parse confidence value!")? 
                            * 100.0
                    )
                );
            },
            _ => {
                email_body.push_str(&format!(
                    "- Status: {:?}<br>- Additional Information: {}<br>",
                    job.status.code,
                    job.status.value
                ));
            }
        }
        /*
        body.push_str(&format!(
            "<h3>Patient Information:</h3>- Age: {}<br>- Ethnicity: {}<br>- Sex: {}<br>- Height: {}<br>- Weight: {}<br><br>",
            job.age,
            job.ethnicity,
            job.sex,
            job.height,
            job.weight
        ));
         */
    }
    print_be!(task_number, "Built the HTML file and email body!");

    // Now that we have created the shortened body, let's 
    //  upload the more verbose file to the user's S3,
    //  and attach the link to the shortened body
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Unreachable - We are not time travelers ^^`")
        .as_secs();

    // Generate the PDF
    let (email, pdf_link) = get_email_and_pdf_link(app, jobs, arguments.uid, timestamp, task_number)
        .await
        .context("Failed to generate the PDF file!")?;

    
    // Add the link to the email body
    email_body += &format!("<br><br><h3>Complete Historical Data:</h3>{}<br>", pdf_link);
    email_body += "<br><br><h2>Please contact &lt;contact email here&gt; with any additional questions!</h2>";

    // Send the email to the email in the first job
    //  (This is a bit of a hack, but it's the easiest way
    //   to send an email while maintaining flexibility)
    send_email(
        &email,
        "Your iGait Submission History",
        &email_body,
        task_number
    ).context("Failed to send email!")?;
    

    Ok("OK")
}
pub async fn get_email_and_pdf_link(
    app: Arc<Mutex<AppState>>,
    jobs: Vec<Job>,
    uid: String,
    timestamp: u64,
    task_number: JobTaskID
) -> Result<(String, String)> {
    let jobs_og = Arc::new(jobs.clone());
    let uid_og = Arc::new(uid);
    let timestamp_og = Arc::new(timestamp);

    let jobs_arc = jobs_og.clone();
    let uid_arc = uid_og.clone();
    let timestamp_arc = timestamp_og.clone();

    let sync_thread = tokio::task::spawn( async move {
        let font_family = genpdf::fonts::from_files("pdf_handling/fonts/SourceSansPro", "SourceSansPro", None)
            .expect("Closure - Failed to load font family!");// Create a document and set the default font family
        let mut doc = genpdf::Document::new(font_family);

        // Decorate
        let mut decorator = genpdf::SimplePageDecorator::new();
        decorator.set_margins(10);

        // Change the default settings
        doc.set_title("Demo document");
        doc.set_page_decorator(decorator);
        
        // Add the body
        let title = genpdf::style::StyledString::new("iGait - Complete Historical Results".to_owned(), genpdf::style::Effect::Bold);
        doc.push(genpdf::elements::Paragraph::new(title));



        // Add each result as its own paragraph
        for (index, job) in jobs_arc.iter().enumerate() {
            let dt_timestamp_utc: DateTime<Utc> = job.timestamp.into();
            let status = match job.status.code {
                JobStatusCode::Complete => {
                    format!(
                        "Complete - Confidence: {:.2}%", 
                        job.status.value
                            .parse::<f64>()
                            .expect("Closure - Failed to parse confidence value!")
                            * 100.0
                    )
                },
                _ => {
                    format!(
                        "{:?} - Additional Information: {}",
                        job.status.code,
                        job.status.value
                    )
                }
            };

            // Add basic data
            doc.push(genpdf::elements::Text::new(genpdf::style::StyledString::new(
                format!("Submission #{}", index +1),
                genpdf::style::Style::new()
            )));
            doc.push(genpdf::elements::Text::new(genpdf::style::StyledString::new(
                format!("Submitted on: {}", dt_timestamp_utc.format("%Y-%m-%d %H:%M:%S")),
                genpdf::style::Style::new()
            )));
            doc.push(genpdf::elements::Paragraph::new(genpdf::style::StyledString::new(
                format!("Status: {}", status),
                genpdf::style::Style::new()
            )));
            doc.push(genpdf::elements::Text::new(genpdf::style::StyledString::new(
                format!("Email: {}", job.email),
                genpdf::style::Style::new()
            )));

            // Add patient data
            doc.push(genpdf::elements::Text::new(genpdf::style::StyledString::new(
                "Patient Information:".to_owned(),
                genpdf::style::Style::new()
            )));
            doc.push(genpdf::elements::Text::new(genpdf::style::StyledString::new(
                format!("Age: {}", job.age),
                genpdf::style::Style::new()
            )));
            doc.push(genpdf::elements::Text::new(genpdf::style::StyledString::new(
                format!("Sex: {}", job.sex),
                genpdf::style::Style::new()
            )));
            doc.push(genpdf::elements::Text::new(genpdf::style::StyledString::new(
                format!("Height: {}", job.height),
                genpdf::style::Style::new()
            )));
            doc.push(genpdf::elements::Text::new(genpdf::style::StyledString::new(
                format!("Weight: {}", job.weight),
                genpdf::style::Style::new()
            )));
            doc.push(genpdf::elements::Text::new(genpdf::style::StyledString::new(
                "",
                genpdf::style::Style::new()
            )));
        }
        print_be!(task_number, "Built PDF file!");

        // Render the file to a compatible Writer
        let path = format!("pdf_handling/history_requests/{}_{}.html", uid_arc, timestamp_arc);
        doc.render_to_file(&path)
            .expect("Closure - Failed to render PDF!");

        print_be!(task_number, "Rendered PDF file to {path}");
    });

    // Await the sync thread
    sync_thread
        .await
        .map_err(|e| anyhow!("{e:#?}"))
        .context("Failed to generate PDF file!")?;
    print_be!(task_number, "Preparing to upload...");

    // Read the bytes of the file
    let path = format!("pdf_handling/history_requests/{}_{}.html", uid_og, timestamp_og);
    print_be!(task_number, "Reading file from {path}");
    let extended_body_byte_vec = tokio::fs::read(&path)
        .await
        .context("Failed to read the PDF file!")?;

    // Remove the file
    tokio::fs::remove_file(&format!("pdf_handling/history_requests/{}_{}.html", uid_og, timestamp_og))
        .await
        .context("Failed to remove the PDF file!")?;
    print_be!(task_number, "Removed file!");

    // Put the extended body into the user's S3 bucket
    print_s3!(task_number, "Putting file to AWS...");
    let aws_path = format!("{}/history_requests/{}.pdf", uid_og, timestamp_og);
    app.lock()
        .await
        .bucket
        .put_object(&aws_path, &extended_body_byte_vec)
        .await 
        .context("Failed to upload front file to S3! Continuing regardless.")?;
    print_s3!(task_number, "Uploaded PDF file to S3!");

    // Generate the presigned URL
    print_s3!(task_number, "Generating presigned URL...");
    let extended_body_url = app.lock()
            .await
            .bucket
            .presign_get(aws_path, 86400 * 7, None)
            .context("Failed to get the front keyframed URL!")?;
    print_s3!(task_number, "Generated a presigned URL for the HTML file!");
    
    let email = jobs
        .iter()
        .next()
        .ok_or(anyhow!("User has no jobs!"))?
        .email
        .clone();
    Ok((email, extended_body_url))
}