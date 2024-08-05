mod helper;
mod daemons;
mod routes;

use anyhow::{ Context, Result };
use axum::{
    extract::DefaultBodyLimit, routing::post, Router
};
use daemons::state::work_queue;
use helper::lib::AppState;
use std::sync::Arc;
use tokio::sync::Mutex;


#[tokio::main]
async fn main() -> Result<()> {
    // Create a thread-safe mutex lock to hold the app state
    let state: Arc<Mutex<AppState>> = Arc::new(Mutex::new(
        AppState::new().await.context("Couldn't set up app state!")?
    ));

    // Build the V1 API router
    let api_v1 = Router::new()
        .route("/upload", post(crate::routes::upload::upload_entrypoint) )
        .route("/completion", post(crate::routes::completion::completion_entrypoint))
        .route("/historical_submissions", post(crate::routes::historical::historical_entrypoint))
        .with_state(state.clone());

    // Nest the API into the general app router
    let app = Router::new()
        .nest("/api/v1", api_v1)
        .layer(DefaultBodyLimit::max(500000000));

    // Start the queue worker
    tokio::spawn(work_queue(state));

    // Serve the API
    print_be!(0, "Starting iGait Backend on 3000...");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await
        .context("Couldn't start up listener!")?;
    axum::serve(listener, app).await
        .context("Could't serve the API!")?;

    Ok(())
}
