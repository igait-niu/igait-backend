use std::{sync::Arc, time::SystemTime};

use anyhow::{ Result, Context, anyhow, bail };
use axum::extract::{Multipart, State};
use chrono::{DateTime, Utc};
use time_util::system_time_from_secs;
use tracing::{warn, info};

use crate::helper::{email::send_email, lib::{AppError, AppState, AppStatePtr, Job, JobStatusCode}};

/// The request arguments for the historical submissions endpoint.
struct HistoricalRequestArguments {
    uid: String,
    entries:          Option<usize>,
    result_type:      Option<String>,
    date_range:      (Option<SystemTime>, Option<SystemTime>),
    include_original: bool,
    include_skeleton: bool
}


/// Takes in the `Multipart` request and unpacks the arguments into a `HistoricalRequestArguments` object.
/// 
/// # Fails
/// If any of the fields are missing or if the files are too large.
/// 
/// # Arguments
/// * `multipart` - The `Multipart` object to unpack.
#[tracing::instrument]
async fn unpack_historical_arguments(
    mut multipart: Multipart
) -> Result<HistoricalRequestArguments> {
    // Create placeholders for all fields
    let mut uid_option: Option<String> = None;
    let mut entries_option: Option<usize> = None;
    let mut result_type_option: Option<String> = None;
    let mut date_range: (Option<SystemTime>, Option<SystemTime>) = (None, None);
    let mut include_original_option = None;
    let mut include_skeleton_option = None;

    while let Some(field) = multipart
        .next_field().await
        .context("Bad request! Is it possible you submitted a file over the size limit?")?
    {
        let name = field.name();
        info!("Field Incoming: {name:#?}");
        match field.name() {
            Some("user_id") => {
                uid_option = Some(
                    field
                        .text().await
                        .context("Field 'user_id' wasn't readable as text!")?);
            },
            Some("entries") => {
                entries_option = Some(
                    field
                        .text().await
                        .context("Field 'user_id' wasn't readable as text!")?
                        .parse::<usize>()
                        .context("Field 'entries' didn't contain a valid integer!")?);

                    if let Some(entries) = &entries_option {
                        if *entries == 0 {
                            bail!("Field 'entries' must be greater than 0!");
                        }
                    }
            },
            Some("result_type") => {
                result_type_option = Some(field
                    .text().await
                    .context("Field 'result_type' wasn't readable as text!")?);

                if let Some(result_type) = &result_type_option {
                    if result_type != "ASD" && result_type != "NO ASD" {
                        bail!("Field 'result_type' must be either 'ASD' or 'NO ASD'!");
                    }
                }
            },
            Some("start_timestamp") => {
                date_range.0 = Some(system_time_from_secs(
                    serde_json::Value::Number(field
                    .text().await
                    .context("Field 'start_timestamp' wasn't readable as text!")?
                    .parse::<u64>()
                    .context("Couldn't parse field 'start_timestamp' into a 64-bit integer!")?
                    .into()
                )).context("Field 'start_timestamp' was an integer, but not a valid UNIX timestamp!")?);
            },
            Some("end_timestamp") => {
                date_range.1 = Some(system_time_from_secs(
                    serde_json::Value::Number(field
                        .text().await
                        .context("Field 'end_timestamp' wasn't readable as text!")?
                        .parse::<u64>()
                        .context("Couldn't parse field 'end_timestamp' into a 64-bit integer!")?
                        .into()
                )).context("Field 'end_timestamp' was an integer, but not a valid UNIX timestamp!")?);
            },
            Some("include_original") => {
                include_original_option = Some(
                    field
                        .text().await
                        .context("Field 'include_original' wasn't readable as text!")?
                        .parse::<bool>()
                        .context("Field 'include_original' didn't contain a valid boolean!")?);
            },
            Some("include_skeleton") => {
                include_skeleton_option = Some(
                    field
                        .text().await
                        .context("Field 'include_skeleton' wasn't readable as text!")?
                        .parse::<bool>()
                        .context("Field 'include_skeleton' didn't contain a valid boolean!")?);
            },
            other => {
                warn!("Recieved unknown/no field name '{other:?}'!");
            }
        }
    }

    Ok(HistoricalRequestArguments {
        uid: uid_option.ok_or(anyhow!("Missing 'user_id' in request!"))?,
        entries: entries_option,
        result_type: result_type_option,
        date_range: date_range,
        include_original: include_original_option.unwrap_or(false),
        include_skeleton: include_skeleton_option.unwrap_or(false)
    })
}

/// The entrypoint for the historical submissions endpoint.
/// 
/// # Fails
/// * If the arguments cannot be unpacked.
/// * If the jobs cannot be retrieved from the database.
/// * If the email cannot be sent.
/// 
/// # Arguments
/// * `app` - The application state.
/// * `multipart` - The `Multipart` object containing the request.
#[tracing::instrument]
pub async fn historical_entrypoint ( 
    State(app): State<AppStatePtr>,
    multipart: Multipart
) -> Result<(), AppError> {
    let app = app.state;

    // Unpack the arguments
    info!("Unpacking arguments...");
    let arguments = unpack_historical_arguments(multipart).await
        .context("Failed to unpack historical arguments!")?;

    // Get all jobs
    let mut jobs: Vec<(usize, Job)> = app.db
        .lock().await
        .get_all_jobs(
            &arguments.uid
        )
        .await
        .context("Failed to get jobs!")?
        .into_iter()
        .enumerate()
        .collect();

    // Remove the template job
    jobs.remove(0);

    // Filter by result type and start/end date
    jobs = jobs.into_iter()
        .filter(|(_, job)| {
            // Filter by result type
            if let Some(result_type) = &arguments.result_type {
                if result_type == "ASD" {
                    if job.status.code != JobStatusCode::Complete ||
                       job.status.value != "ASD"
                    {
                        return false;
                    }
                } else if result_type == "NO ASD" {
                    if job.status.code != JobStatusCode::Complete || 
                       job.status.value != "NO ASD"
                    {
                        return false;
                    }
                }
            }

            // Make sure it's after the start date
            if let Some(start_date) = arguments.date_range.0 {
                if job.timestamp < start_date {
                    return false;
                }
            }

            // Make sure it's before the end date
            if let Some(end_date) = arguments.date_range.1 {
                if job.timestamp > end_date {
                    return false;
                }
            }

            true
        })
        .collect();

    jobs.reverse();

    // Lastly, only the return the number of entries requested
    if let Some(num_entries) = arguments.entries {
        jobs.truncate(num_entries);
    }
    
    // Generate the body
    let mut email_body = concat!(
        "<h1>Thank you for contacting iGait!</h1>",
        "You recently requested a history of your submissions. Located below can be found, in chronological order, your past submissions.<br><br>",
        "You selected the following filters:"
    ).to_string();
    email_body += &format!(
        "<ul>\
        <li>Entries: {}</li>\
        <li>Result Type: {}</li>\
        <li>Start Date: {}</li>\
        <li>End Date: {}</li>\
        <li>Include Original: {}</li>\
        <li>Include Skeleton: {}</li>\
        </ul><br>",
        arguments.entries.map(|n| n.to_string()).unwrap_or(String::from("All")),
        arguments.result_type.as_deref().unwrap_or("Any"),
        arguments.date_range.0.map(|d| DateTime::<Utc>::from(d).with_timezone(&chrono_tz::US::Central).to_string()).unwrap_or("Any".to_string()),
        arguments.date_range.1.map(|d| DateTime::<Utc>::from(d).with_timezone(&chrono_tz::US::Central).to_string()).unwrap_or("Any".to_string()),
        if arguments.include_original { "Yes" } else { "No" },
        if arguments.include_skeleton { "Yes" } else { "No" }
    );

    // Make sure that we have at least some results
    if jobs.is_empty() {
        return Err(AppError(anyhow!("There were no jobs that matched your query!")));
    }

    for (job_id, job) in jobs.iter() {
        // Add a condensed version of the job to the shortened body
        let dt_timestamp_utc: DateTime<Utc> = job.timestamp.into();
        let dt_timestamp_cst = dt_timestamp_utc.with_timezone(&chrono_tz::US::Central);
        email_body.push_str(&format!(
            "<h2>{}</h2>",
            dt_timestamp_cst.to_string()
        ));
        
        match job.status.code {
            JobStatusCode::Complete => {
                email_body.push_str(
                    &format!(
                        "- Status: Complete<br>- Result: {}<br>", 
                        job.status.value
                    )
                );
                if arguments.include_skeleton {
                    info!("Preparing to grab pre-signed URLs for the skeleton-overlaid videos...");

                    let inputs_prefix = format!(
                        "{}/outputs/{};{}/videos/",
                        arguments.uid,
                        arguments.uid, job_id
                    );
        
                    // List the available files (since we don't know the extensions)
                    let results = app
                        .bucket
                        .lock().await
                        .list(inputs_prefix, Some("/".to_string())).await?;
                    let mut front_file_key = None;
                    let mut side_file_key = None;
                    for object in &results[0].contents {
                        let file = object.key
                            .split("/")
                            .last()
                            .context("Missing filename! (Likely unreachable?)")?;
                        let file_name = file.split(".")
                            .next()
                            .context("Empty filename! (Likely unreachable?)")?;
                        info!("Found file `{file}` with name `{file_name}`");
        
                        if file_name.contains("__F_") {
                            front_file_key = Some(object.key.clone());
                        } else if file_name.contains("__S_") {
                            side_file_key = Some(object.key.clone());
                        }
                    }
        
                    let front_skeleton_video_link = app
                        .bucket
                        .lock().await
                        .presign_get(front_file_key.context("Missing front file!")?, 86400 * 7, None)
                        .context("Failed to get the front keyframed URL!")?;
                    let side_skeleton_video_link = app
                        .bucket
                        .lock().await
                        .presign_get(side_file_key.context("Missing front file!")?, 86400 * 7, None)
                        .context("Failed to get the front keyframed URL!")?;
        
                    email_body.push_str(&format!(
                        "- <a href=\"{}\">Skeleton Front Video</a><br>- <a href=\"{}\">Skeleton Side Video</a><br>",
                        front_skeleton_video_link,
                        side_skeleton_video_link
                    ));

                    info!("Grabbed pre-signed URLs for the skeleton-overlaid videos!");
                }
            },
            _ => {
                email_body.push_str(&format!(
                    "- Status: {:?}<br>- Additional Information: {}<br>",
                    job.status.code,
                    job.status.value
                ));

                if arguments.include_skeleton {
                    email_body.push_str("Since this job does not have a complete result, we are unable to provide a skeletonized video.<br>");
                }
            }
        }
        
        // Add the original videos, if requested
        if arguments.include_original {
            info!("Preparing to grab pre-signed URLs for the original videos...");
            let inputs_prefix = format!(
                "{}/inputs/{};{}/",
                arguments.uid,
                arguments.uid, job_id
            );

            // List the available files (since we don't know the extensions)
            let results = app
                .bucket
                .lock().await
                .list(inputs_prefix, Some("/".to_string())).await?;
            let mut front_file_key = None;
            let mut side_file_key = None;
            for object in &results[0].contents {
                let file = object.key
                    .split("/")
                    .last()
                    .context("Missing filename! (Likely unreachable?)")?;
                let file_name = file.split(".")
                    .next()
                    .context("Empty filename! (Likely unreachable?)")?;
                info!("Found file `{file}` with name `{file_name}`");

                if file_name == "front" {
                    front_file_key = Some(object.key.clone());
                } else if file_name == "side" {
                    side_file_key = Some(object.key.clone());
                }
            }

            let front_original_video_link = app
                .bucket
                .lock().await
                .presign_get(front_file_key.context("Missing front file!")?, 86400 * 7, None)
                .context("Failed to get the front keyframed URL!")?;
            let side_original_video_link = app
                .bucket
                .lock().await
                .presign_get(side_file_key.context("Missing front file!")?, 86400 * 7, None)
                .context("Failed to get the front keyframed URL!")?;

            email_body.push_str(&format!(
                "- <a href=\"{}\">Original Front Video</a><br>- <a href=\"{}\">Original Side Video</a><br>",
                front_original_video_link,
                side_original_video_link
            ));

            info!("Grabbed pre-signed URLs for the original videos!");
        }
    }
    info!("Built the HTML file and email body!");

    // Now that we have created the shortened body, let's 
    //  upload the more verbose file to the user's S3,
    //  and attach the link to the shortened body
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .context("Unreachable - We are not time travelers ^^`")?
        .as_secs();

    // Generate the PDF
    let (email, pdf_link) = get_email_and_pdf_link(app.clone(), jobs, arguments.uid, timestamp)
        .await
        .context("Failed to generate the PDF file!")?;

    
    // Add the link to the email body
    email_body += &format!("<br><br><h3>Complete Historical Data</h3><p>If you would like to see patient data for each submission, please click <a href=\"{}\">here</a></p><br>", pdf_link);
    email_body += "<br><br><h2>Please contact &lt;contact email here&gt; with any additional questions!</h2>";

    // Send the email to the email in the first job
    //  (This is a bit of a hack, but it's the easiest way
    //   to send an email while maintaining flexibility)
    send_email(
        app,
        &email,
        "Your iGait Submission History",
        &email_body
    ).await
        .context("Failed to send email!")?;

    Ok(())
}

/// Generates a PDF file containing the complete history of the user's submissions.
/// 
/// # Fails
/// * If the PDF cannot be generated.
/// * If the PDF cannot be read.
/// * If the PDF cannot be uploaded to the user's S3 bucket.
/// * If the presigned URL cannot be generated.
/// 
/// # Panics
/// * If the PDF cannot be rendered.
/// 
/// # Arguments
/// * `app` - The application state.
/// * `jobs` - The jobs to include in the PDF.
/// * `uid` - The user ID.
/// * `timestamp` - The timestamp to use for the PDF.
/// 
/// # Returns
/// * The email address of the user.
/// * The presigned URL of the PDF.
/// 
/// # Notes
/// <div class="warning">
///    This function is currently a bit of a hack. It uses a synchronous closure to generate the PDF.
///    <br>It is possible for this function to panic without catching the panic.
///    <br><br>Currently, I do not have the technical skills to fix this. I will come back with more skill later.
/// </div>
#[tracing::instrument]
async fn get_email_and_pdf_link(
    app: Arc<AppState>,
    jobs: Vec<(usize, Job)>,
    uid: String,
    timestamp: u64
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
        for (index, job) in jobs_arc.iter() {
            let dt_timestamp_utc: DateTime<Utc> = job.timestamp.into();
            let dt_timestamp_cst = dt_timestamp_utc.with_timezone(&chrono_tz::US::Central);
            let status = match job.status.code {
                JobStatusCode::Complete => {
                    format!(
                        "Complete - Result: {}", 
                        job.status.value
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
                format!("Submitted on: {}", dt_timestamp_cst.to_string()),
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
        info!("Built PDF file!");

        // Render the file to a compatible Writer
        let path = format!("pdf_handling/history_requests/{}_{}.html", uid_arc, timestamp_arc);
        doc.render_to_file(&path)
            .expect("Closure - Failed to render PDF!");

        info!("Rendered PDF file to {path}");
    });

    // Await the sync thread
    sync_thread
        .await
        .map_err(|e| anyhow!("{e:#?}"))
        .context("Failed to generate PDF file!")?;
    info!("Preparing to upload...");

    // Read the bytes of the file
    let path = format!("pdf_handling/history_requests/{}_{}.html", uid_og, timestamp_og);
    info!("Reading file from {path}");
    let extended_body_byte_vec = tokio::fs::read(&path)
        .await
        .context("Failed to read the PDF file!")?;

    // Remove the file
    tokio::fs::remove_file(&format!("pdf_handling/history_requests/{}_{}.html", uid_og, timestamp_og))
        .await
        .context("Failed to remove the PDF file!")?;
    info!("Removed file!");

    // Put the extended body into the user's S3 bucket
    info!("Putting file to AWS...");
    let aws_path = format!("{}/history_requests/{}.pdf", uid_og, timestamp_og);
    app.bucket
        .lock().await
        .put_object(&aws_path, &extended_body_byte_vec)
        .await 
        .context("Failed to upload front file to S3! Continuing regardless.")?;
    info!("Uploaded PDF file to S3!");

    // Generate the presigned URL
    info!("Generating presigned URL...");
    let extended_body_url = app
            .bucket
            .lock().await
            .presign_get(aws_path, 86400 * 7, None)
            .context("Failed to get the front keyframed URL!")?;
    info!("Generated a presigned URL for the HTML file!");
    
    let email = jobs
        .iter()
        .next()
        .ok_or(anyhow!("User has no jobs!"))?
        .1
        .email
        .clone();
    Ok((email, extended_body_url))
}