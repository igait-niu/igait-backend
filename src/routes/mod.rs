use std::fs::{ File };
use std::io::Write;

use axum::extract::{ State, Multipart, Path };
use crate::state::{ AppState };
use crate::request::{ StatusCode };
use crate::{
    Arc, Mutex
};

/* Primary Routes */
pub async fn upload(State(app): State<Arc<Mutex<AppState>>>, multipart: Multipart) -> String {
    /*
    app.lock().await
        .db
        .new_entry()
    */

    todo!()
}

/* Helper Functions */
async fn add_file_to_queue(app: Arc<Mutex<AppState>>, mut multipart: Multipart, id: usize) -> Result<StatusCode, String> {
    // Iter fields
    while let Some(field) = multipart
        .next_field().await
        .map_err(|_| {
            String::from("Malformed multipart request")
        })?
    {
        // Verify file upload
        let field_type = field
            .name().unwrap();
        if field_type != "fileupload" {
            println!("[Invalid Field Type '{}']", field_type);
            continue;
        }

        // Verify extension type
        let file_name = field
            .file_name()
            .ok_or_else(|| {
                String::from("Must have associated file name in multipart!")
            })?;
        let extension = file_name.split(".")
            .nth(1)
            .ok_or_else(|| {
                String::from("Must have a file extension!")
            })?;
        if extension != "mp4" {
            return Err(String::from("Must be of filetype MP4!"));
        }
        
        // Grab byte data
        let data = field.bytes()
            .await
            .map_err(|_| {
                String::from("Could not unwrap bytes!")
            })?;
        
        // Build path ID and file handle
        let queue_file_path = format!("data/queue/{}.mp4", id);
        let mut queue_file_handle = File::create(queue_file_path)
            .unwrap();

        // Write data
        queue_file_handle.write_all(&data.clone())
            .map_err(|_| String::from("Unable to write file!"))?;

        // Build path ID and file handle
        let resources_file_path = format!("data/resources/{}.mp4", id);
        let mut resources_file_handle = File::create(resources_file_path)
            .unwrap();

        // Write data
        resources_file_handle.write_all(&data)
            .map_err(|_| String::from("Unable to write file!"))?;
    }

    tokio::spawn(AppState::work_queue(app.clone()));

    Ok(StatusCode::Queue)
}