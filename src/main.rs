#![doc = include_str!("./docs/home.md")]
mod helper;
mod daemons;
mod routes;

use anyhow::{ Context, Result };
use axum::{
    extract::DefaultBodyLimit, routing::{any, post}, Router
};
use daemons::filesystem::work_inputs;
use helper::{lib::AppState, metis::{copy_file, SSHPath, METIS_DATA_DIR, METIS_HOSTNAME, METIS_USERNAME}};
use std::sync::Arc;

pub const ASD_CLASSIFICATION_THRESHOLD: f32 = 0.5;

/// The main entrypoint for the iGait backend.
/// 
/// # Fails
/// * If the app state fails to initialize
/// * If the listener fails to start
/// * If the API fails to serve
/// 
/// # Returns
/// * A successful result if the API is served
/// 
/// # Notes
/// * The API is served on port 3000
/// * The API is served at the root of the server
/// * The API is served with a body limit of 500MB
/// * The API is served with the V1 API nested under `/api/v1`
#[tokio::main]
async fn main() -> Result<()> {
    // Create a thread-safe mutex lock to hold the app state
    let state: Arc<AppState> = Arc::new(
        AppState::new().await.context("Couldn't set up app state!")?
    );

    // Build the V1 API router
    let api_v1 = Router::new()
        .route("/upload", post(crate::routes::upload::upload_entrypoint) )
        .route("/historical_submissions", post(crate::routes::historical::historical_entrypoint))
        .route("/assistant", any(crate::routes::assistant::assistant_entrypoint))
        .with_state(state.clone());

    // Nest the API into the general app router
    let app = Router::new()
        .nest("/api/v1", api_v1)
        .layer(DefaultBodyLimit::max(500000000));

    // Copy scripts to Metis
    print_metis!(0, "Copying scripts from local to Metis...");
    copy_file(
        METIS_USERNAME,
        METIS_HOSTNAME,
        SSHPath::Local("scripts"),
        SSHPath::Remote(METIS_DATA_DIR),
        true
    ).await
        .context("Couldn't move the outputs from Metis to local outputs directory!")?;
    print_metis!(0, "Successfully copied scripts from local to Metis!");

    // Start the inputs worker
    tokio::spawn(work_inputs(state));

    // Serve the API
    let port = std::env::var("PORT").unwrap_or("3000".to_string());
    print_be!(0, "Starting iGait Backend on {port}...");
    let listener = tokio::net::TcpListener::bind(&format!("0.0.0.0:{port}")).await
        .context("Couldn't start up listener!")?;
    axum::serve(listener, app).await
        .context("Could't serve the API!")?;

    Ok(())
}
