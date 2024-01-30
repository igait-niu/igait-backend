mod database;
mod state;
mod inference;
mod request;
mod routes;

use axum::{
    routing::{ get, post },
    response::{ Html },
    Router,
};
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    // Build the general app state
    let state: Arc<Mutex<state::AppState>> = Arc::new(
        Mutex::new(
            state::AppState::new().await
        )
    );

    // Build the V1 API router
    let api_v1 = Router::new()
        .route("/upload", post(routes::upload) )
        .with_state(state.clone());

    // Nest the API into the general app router
    let app = Router::new()
        .route("/", get(|| async { Html(std::include_str!("../public/index.html")) }))
        .nest("/api/v1", api_v1)
        .nest_service("/public", ServeDir::new("public"))
        .layer(LiveReloadLayer::new());

    // Start the queue worker
    tokio::spawn(state::work_queue(state));

    // Serve the API
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
