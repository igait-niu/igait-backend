mod database;
mod state;
mod inference;
mod request;
mod routes;

use axum::{
    routing::{ get, post },
    Router,
};
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    // Build the general app state
    let state: Arc<Mutex<state::AppState>> = Arc::new(
        Mutex::new(
            state::AppState::new()
        )
    );

    // Bulid the V1 API router
    let api_v1 = Router::new()
        .route("/status/:id", get(routes::status))
        .route("/upload", post(routes::upload) )
        .with_state(state);

    // Nest the API into the general app router
    let app = Router::new()
        .nest("/api/v1", api_v1);

    // Serve the API
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}