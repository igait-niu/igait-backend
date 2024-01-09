use std::fs::{ File };
use std::io::Write;

use axum::extract::{ Path, State, Multipart };
use crate::state::{ AppState };
use crate::request::{ Status };
use crate::{
    Arc, Mutex
};

/* Primary Routes */
pub async fn upload(State(app): State<Arc<Mutex<AppState>>>, multipart: Multipart) -> String {
    // Generate job ID
    let id = app.lock().await
        .db
        .new_entry();

    let result = add_file_to_queue(app.clone(), multipart, id).await
        .unwrap_or_else(|err| Status::SubmissionErr(err));
    
    AppState::update_status(&app, id, result).await;
    
    id.to_string()
}
pub async fn status(Path(id): Path<usize>, State(app): State<Arc<Mutex<AppState>>>) -> String {
    app
        .lock().await
        .db
        .get(id)
        .and_then(|request| Some(format!("{}", request.status)))
        .unwrap_or(String::from("Unable to find ID!"))
}

/* Helper Functions */
async fn add_file_to_queue(app: Arc<Mutex<AppState>>, mut multipart: Multipart, id: usize) -> Result<Status, String> {
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
        let file_path = format!("data/queue/{}.mp4", id);
        let mut file_handle = File::create(file_path)
            .unwrap();

        // Write data
        file_handle.write_all(&data)
            .map_err(|_| String::from("Unable to write file!"))?;
    }

    tokio::spawn(AppState::work_queue(app.clone()));

    Ok(Status::Queue)
}