#![doc = include_str!("./docs/home.md")]
mod helper;
mod routes;

use anyhow::{ Context, Result };
use axum::{
    extract::DefaultBodyLimit, routing::{any, post}, Router
};
use helper::lib::{AppState, AppStatePtr};
use std::sync::Arc;
use tracing_subscriber;
use dotenv::dotenv;

pub const ASD_CLASSIFICATION_THRESHOLD: f32 = 0.5;
pub const DISABLE_RESULT_EMAIL: bool = true;

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
#[tracing::instrument]
async fn main() -> Result<()> {
    
    // Enable loading on WSL
    dotenv().ok();

    // Initialize the logger
    tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Create a thread-safe mutex lock to hold the app state
    let state: Arc<AppState> = Arc::new(
        AppState::new().await.context("Couldn't set up app state!")?
    );
    let app_state_ptr = AppStatePtr { state: state.clone() };

    // Build the V1 API router
    let api_v1 = Router::new()
        .route("/upload", post(crate::routes::upload::upload_entrypoint) )
        .route("/contribute", post(crate::routes::contribute::contribute_entrypoint))
        .route("/assistant", any(crate::routes::assistant::assistant_entrypoint))
        .route("/assistant_proxied", any(crate::routes::assistant::assistant_proxied_entrypoint))
        .route("/webhook/stage/{stage_num}", post(crate::routes::webhook::stage_webhook_entrypoint))
        .with_state(app_state_ptr);

    // Nest the API into the general app router
    let app = Router::new()
        .nest("/api/v1", api_v1)
        .layer(DefaultBodyLimit::max(500000000));

    // Serve the API
    let port = std::env::var("PORT").unwrap_or("3000".to_string());
    println!("Starting iGait backend on port {port}...");
    let listener = tokio::net::TcpListener::bind(&format!("0.0.0.0:{port}")).await
        .context("Couldn't start up listener!")?;
    axum::serve(listener, app).await
        .context("Could't serve the API!")?;

    Ok(())
}
