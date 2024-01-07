mod database;
mod state;
mod inference;
mod request;

use request::{ Status };
use state::{ AppState };
use axum::{
    routing::{ get, post },
    extract::{ Path, State },
    Router,
};
use std::sync::Arc;
use tokio::sync::Mutex;

async fn upload(State(app): State<Arc<Mutex<AppState>>>) -> Result<String, String> {
    // Create ID
    let id = app
        .lock().await
        .db
        .new_entry();
    

    // Save file
    // write_to_file(name: id);

    state::AppState::update_status(&app, id, Status::Submitted).await;

    // Update queue
    app.lock().await
        .queue
        .push_back(id);
    
    state::AppState::update_status(&app, id, Status::Queue).await;

    tokio::spawn(state::AppState::work_queue(app.clone()));
    
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