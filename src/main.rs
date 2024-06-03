mod database;
mod state;
mod request;
mod routes;
mod print;
mod email;

use crate::print::*;
use axum::{
    routing::{ post },
    extract::DefaultBodyLimit,
    Router
};
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    // Create a thread-safe mutex lock to hold the app state
    let state: Arc<Mutex<state::AppState>> = Arc::new(
        Mutex::new(
            state::AppState::new().await
        )
    );

    // Build the V1 API router
    let api_v1 = Router::new()
        .route("/upload", post(routes::upload) )
        .route("/completion", post(routes::completion))
        .with_state(state.clone());

    // Nest the API into the general app router
    let app = Router::new()
        .nest("/api/v1", api_v1)
        .layer(DefaultBodyLimit::max(500000000));

    // Start the queue worker
    tokio::spawn(state::work_queue(state));

    print_be("Started iGait Backend on 3000!");

    // Serve the API
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
