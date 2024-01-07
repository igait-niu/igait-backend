mod database;
mod state;
mod inference;
mod request;

use axum::{
    routing::{ get, post },
    extract::{ Path, State },
    Router,
};
use std::sync::Arc;
use tokio::sync::Mutex;

async fn upload(State(app): State<Arc<Mutex<state::AppState>>>) -> Result<String, String> {
    // Create ID
    let id = app
        .lock().await
        .db
        .new_entry();

    // Save file
    // write_to_file(name: id);
    app.lock().await
        .db
        .get(id).ok_or("ID not found. Request may have been deleted.")?
        .status = request::Status::Submitted;

    // Update queue
    app.lock().await
        .queue
        .push_back(id);

    let mut app_lock = app.lock().await;
    app_lock.db
            .get(id).ok_or("ID not found. request::Request may have been deleted.")?
            .status = request::Status::Queue;
    drop(app_lock);

    tokio::spawn(state::AppState::work_queue(app.clone()));
    
    println!("[New Entry: ID {id}]");
    Ok(id.to_string())
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
        .route("/upload", post(upload).get(upload) )// GET IS TEMPORARY
        .with_state(state);

    let app = Router::new()
        .nest("/api/v1", api_v1);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}