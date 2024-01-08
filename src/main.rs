mod database;
mod state;
mod inference;
mod request;

use request::{ Status };
use state::{ AppState };
use axum::{
    routing::{ get, post },
    extract::{ Path, State, Multipart },
    Router,
};
use std::fs::{ File, remove_file };
use std::io::Write;
use std::sync::Arc;
use tokio::sync::Mutex;

async fn add_file_to_queue(app: Arc<Mutex<AppState>>, mut multipart: Multipart, id: usize) -> Result<Status, String> {
    // Iter fields
    while let Some(mut field) = multipart
        .next_field().await
        .map_err(|_| {
            String::from("Malformed multipart request")
        })?
    {
        // Grab extension and byte data
        let extension = field
            .name().unwrap()
            .split(".")
            .nth(1)
            .ok_or_else(|| {
                String::from("File must have a valid extension!")
            })?
            .to_string();
        let data = field.bytes()
            .await
            .map_err(|_| {
                String::from("Could not unwrap bytes!")
            })?;

        // Build path ID and file handle
        let file_path = format!("data/queue/{}.{}", id, extension);
        let mut file_handle = File::create(file_path)
            .unwrap();

        // Write data
        file_handle.write_all(&data)
            .map_err(|_| String::from("Unable to write file!"))?;
    }

    // Update queue
    app.lock().await
        .queue
        .push_back(id);

    tokio::spawn(state::AppState::work_queue(app.clone()));

    Ok(Status::Queue)
}
async fn upload(State(app): State<Arc<Mutex<AppState>>>, multipart: Multipart) -> String {
    // Generate job ID
    let id = app.lock().await
        .db
        .new_entry();

    let result = add_file_to_queue(app.clone(), multipart, id).await
        .unwrap_or_else(|err| Status::SubmissionErr(err));
    
    AppState::update_status(&app, id, result).await;
    
    id.to_string()
}
async fn status(Path(id): Path<usize>, State(app): State<Arc<Mutex<state::AppState>>>) -> String {
    app
        .lock().await
        .db
        .get(id)
        .and_then(|request| Some(format!("{:?}", request.status)))
        .unwrap_or(String::from("Unable to find ID!"))
}
#[tokio::main]
async fn main() {
    let state: Arc<Mutex<state::AppState>> = Arc::new(
        Mutex::new(
            state::AppState::new()
        )
    );

    let api_v1 = Router::new()
        .route("/status/:id", get(status))
        .route("/upload", post(upload) )
        .with_state(state);

    let app = Router::new()
        .nest("/api/v1", api_v1);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}