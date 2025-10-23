use std::{sync::Arc, time::SystemTime};

use axum::{body::Bytes, extract::{Multipart, State}};
use tokio::io::AsyncWriteExt;
use anyhow::{ Result, Context, anyhow };
use tracing::info;

use crate::helper::{email::send_contribution_email, lib::{AppError, AppState, AppStatePtr}};

/// A request to upload a video for the contribute endpoint.
pub struct ContributeRequestArguments {
    uid: String,
    name: String,
    email: String,
    front_file: ContributeRequestFile,
    side_file:  ContributeRequestFile,
}

/// A representation of a file in a `Multipart` request.
#[derive(Debug)]
struct ContributeRequestFile {
    name:  String,
    bytes: Bytes
}


/// Takes in the `Multipart` request and unpacks the arguments into a `ContributeRequestArguments` object.
/// 
/// # Fails
/// If any of the fields are missing or if the files are too large.
/// 
/// # Arguments
/// * `multipart` - The `Multipart` object to unpack.
#[tracing::instrument]
async fn unpack_contribute_arguments(
    multipart:   &mut Multipart
) -> Result<ContributeRequestArguments> {
    // Initialize all of the fields as options
    let mut uid_option:       Option<String> = None;
    let mut name_option:      Option<String> = None;
    let mut email_option:     Option<String> = None;

    // Initialize the file fields as options
    let mut front_file_name_option:  Option<String> = None;
    let mut side_file_name_option:   Option<String> = None;
    let mut front_file_bytes_option: Option<Bytes>  = None;
    let mut side_file_bytes_option:  Option<Bytes>  = None;

    // Loop through the fields
    while let Some(field) = multipart
        .next_field().await
        .context("Bad upload request! Is it possible you submitted a file over the size limit?")?
    {
        let name = field.name();
        let field_name = field.file_name();
        info!("Field Incoming: {name:?} - File Attached: {field_name:?}");
        
        match field.name() {
            Some("fileuploadfront") => {
                front_file_name_option = field
                    .file_name().map(|x| String::from(x));
                front_file_bytes_option = Some(field.bytes()
                    .await
                    .context("Could not unpack bytes from field 'fileuploadfront'! Was there no file attached?")?);
            },
            Some("fileuploadside") => {
                side_file_name_option = field
                    .file_name().map(|x| String::from(x));
                side_file_bytes_option = Some(field.bytes()
                    .await
                    .context("Could not unpack bytes from field 'fileuploadside'! Was there no file attached?")?);
            },
            Some("email") => {
                email_option = Some(
                    field
                        .text().await
                        .context("Field 'uid' wasn't readable as text!")?
                        .to_string());
            }
            Some("name") => {
                name_option = Some(
                    field
                        .text().await
                        .context("Field 'name' wasn't readable as text!")?
                        .to_string());
            }
            Some("uid") => {
                uid_option = Some(
                    field
                        .text().await
                        .context("Field 'uid' wasn't readable as text!")?
                        .to_string());
            }
            _ => {
                info!("Which had an unknown/no field name...");
            }
        }
    }

    // Make sure all of the fields are present
    let uid:   String = uid_option.ok_or(   anyhow!( "Missing 'uid' in request"   ))?;
    let name:  String = name_option.ok_or( anyhow!( "Missing 'name' in request" ))?;
    let email: String = email_option.ok_or( anyhow!( "Missing 'email' in request" ))?;

    // Make sure all of the file fields are present
    let front_file_name:  String = front_file_name_option.ok_or(  anyhow!( "Missing 'fileuploadfront' in request!" ))?;
    let side_file_name:   String = side_file_name_option.ok_or(   anyhow!( "Missing 'fileuploadside' in request!"  ))?;
    let front_file_bytes: Bytes  = front_file_bytes_option.ok_or( anyhow!( "Missing 'fileuploadfront' in request!" ))?;
    let side_file_bytes:  Bytes  = side_file_bytes_option.ok_or(  anyhow!( "Missing 'fileuploadside' in request!"  ))?;

    Ok(ContributeRequestArguments {
        uid,
        name,
        email, 
        front_file: ContributeRequestFile {
            name: front_file_name, 
            bytes: front_file_bytes
        },
        side_file: ContributeRequestFile {
            name: side_file_name,
            bytes: side_file_bytes
        }
    })
}

/// The entrypoint for the contribute request.
/// 
/// # Fails
/// * If the arguments are missing.
/// * If the files are too large.
/// * If the files fail to save to S3.
/// * If the job fails to save to the database.
/// * If the welcome email fails to send.
/// 
/// # Arguments
/// * `app` - The application state.
/// * `multipart` - The `Multipart` object to unpack.
#[tracing::instrument]
pub async fn contribute_entrypoint(
    State(app): State<AppStatePtr>,
    mut multipart: Multipart
) -> Result<(), AppError> {
    let app = app.state;

    info!("Unpacking arguments...");
    // Unpack the arguments
    let arguments: ContributeRequestArguments = unpack_contribute_arguments(
            &mut multipart
        ).await
        .context("Failed to unpack arguments!")?;

    // Try to save the files to S3
    if let Err(err) = 
        save_upload_files( 
            app.clone(),
            arguments.front_file,
            arguments.side_file,
            &arguments.uid,
            &arguments.email,
            &arguments.name
        ).await 
    {
        return Err(AppError(err
            .context("Failed to save locally or upload files to S3!")));
    }

    // Thank the user for their contribution
    send_contribution_email(
        app.clone(),
        &arguments.email,
        &arguments.name
    )
        .await
        .context("Failed to send contribution email!")?;
    info!("Successfully sent contribution email!");

    Ok(())
}

/// Saves the upload files to S3 and the local filesystem.
/// 
/// # Fails
/// * If the files fail to save to S3.
/// * If the files fail to save to the local filesystem.
/// 
/// # Arguments
/// * `app` - The application state.
/// * `front_file` - The front file to save.
/// * `side_file` - The side file to save.
/// * `user_id` - The user ID to save the files under.
/// * `email` - The email to save the files under.
/// * `name` - The name to save the files under.
#[tracing::instrument]
async fn save_upload_files<'a> (
    app:              Arc<AppState>,
    front_file:       ContributeRequestFile,
    side_file:        ContributeRequestFile,
    user_id:          &str,
    email:            &str,
    name:             &str,
) -> Result<()> {
    // Unpack the extensions
    let front_extension = front_file.name.split('.')
        .last()
        .context("Must have a file extension!")?;
    let side_extension = side_file.name.split('.')
        .last()
        .context("Must have a file extension!")?;
    
    // Ensure a directory exists for this file ID
    let unix_timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .context("Failed to get the current time!")?
        .as_secs();
    let email_user_id = format!("{};{}", user_id, email.replace('@', "_at_"));

    // Build byte vectors
    let mut front_byte_vec: Vec<u8> = Vec::new();
    let mut side_byte_vec: Vec<u8> = Vec::new();
    front_byte_vec.write_all(&front_file.bytes)
        .await
        .context("Failed to build u8 vector from the front file's Bytes object!")?;
    side_byte_vec.write_all(&side_file.bytes)
        .await
        .context("Failed to build u8 vector from side file's Bytes object!")?;

    // Upload the all three files to S3
    app.bucket
        .lock().await
        .put_object(format!("research/{}/{}/front.{}", email_user_id, unix_timestamp, front_extension), &front_byte_vec)
        .await 
        .context("Failed to upload front file to S3! Continuing regardless.")?;
    info!("Successfully uploaded front file to S3!");
    app.bucket
        .lock().await
        .put_object(format!("research/{}/{}/side.{}", email_user_id, unix_timestamp, side_extension), &side_byte_vec)
        .await
        .context("Failed to upload front side to S3! Continuing regardless.")?;
    info!("Successfully uploaded side file to S3!");
    info!("Successfully saved all files physically and to S3!");
    
    // Return as successful
    Ok(())
}
