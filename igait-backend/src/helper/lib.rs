use std::{sync::Arc, time::SystemTime};

use anyhow::{ Result, Context };
use axum::{
    async_trait, body::Body, extract::FromRequestParts, http::{self, request::Parts}, response::{IntoResponse, Response}
};
use serde::{Deserialize, Serialize};
use async_openai::{
    config::OpenAIConfig, types::AssistantObject, Client
};
use tokio::sync::Mutex;
use firebase_auth::{FirebaseAuth, FirebaseUser};
use igait_lib::microservice::{EmailClient, StorageClient};
use ts_rs::TS;

use super::database::Database;

/// Custom serialization module for SystemTime as Unix timestamp (seconds)
mod systemtime_as_secs {
    use std::time::{SystemTime, UNIX_EPOCH};
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(time: &SystemTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let duration = time.duration_since(UNIX_EPOCH)
            .map_err(serde::ser::Error::custom)?;
        serializer.serialize_u64(duration.as_secs())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<SystemTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let secs = u64::deserialize(deserializer)?;
        Ok(UNIX_EPOCH + std::time::Duration::from_secs(secs))
    }
}

/// The user struct, which contains a user ID and a list of jobs.
/// 
/// # Fields
/// * `uid` - The user ID
/// * `jobs` - The list of jobs
/// * `administrator` - Whether the user has administrator privileges
#[derive( Serialize, Deserialize, Debug, TS )]
#[ts(export)]
pub struct User {
    pub uid: String,
    pub jobs: Vec<Job>,
    #[serde(default)]
    pub administrator: bool,
}

/// The job struct, which contains the job
/// 
/// # Fields
/// * `age` - The age of the patient
/// * `ethnicity` - The ethnicity of the patient
/// * `sex` - The assigned sex of the patient
/// * `height` - The height of the patient
/// * `status` - The status of the job
/// * `timestamp` - The timestamp of the job (Unix timestamp in seconds)
/// * `weight` - The weight of the patient
/// * `email` - The email of the person who submitted the job
#[derive( Serialize, Deserialize, Clone, Debug, TS )]
#[ts(export)]
pub struct Job {
    pub age: i16,
    pub ethnicity: String,
    pub sex: char,
    pub height: String,
    pub status: JobStatus,
    #[serde(with = "systemtime_as_secs")]
    #[ts(type = "number")]
    pub timestamp: SystemTime,
    pub weight: i16,
    pub email: String
}

/// The job status struct, which contains the status of the job.
/// 
/// # Fields
/// * `code` - The status code of the job
/// * `value` - The human-readable value of the status
#[derive( Serialize, Deserialize, Clone, Debug, TS )]
#[ts(export)]
pub struct JobStatus {
    pub code: JobStatusCode,
    pub value: String,
}

/// The job status code enum, which contains the status codes for the job.
/// 
/// # Variants
/// * `Submitting` - The job is downloading on AWS
/// * `SubmissionErr` - The job download has errored on AWS
/// * `Queue` - The job is in the queue on AWS to be sent to Metis
/// * `Processing` - The job is processing on Metis
/// * `InferenceErr` - The job has errored during inference on Metis
/// * `Complete` - The job has completed successfully
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, TS)]
#[ts(export)]
pub enum JobStatusCode {
    Submitting,
    SubmissionErr,
    Queue,
    Processing,
    InferenceErr,
    Complete
}

/// The state of the entire backend application with handles to the database and storage.
/// 
/// # Fields
/// * `db` - The database handle (Firebase RTDB)
/// * `storage` - Firebase Storage client (GCS-backed)
/// * `email_client` - Email client for sending notifications
/// * `openai_client` - OpenAI client for AI assistant
/// * `openai_assistant` - The loaded OpenAI assistant
/// * `firebase_auth` - Firebase Auth for user verification
/// 
/// # Notes
/// * This struct is typically wrapped in an `Arc<>` to allow for concurrent access.
impl std::fmt::Debug for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppState")
            .field("db", &self.db)
            .field("storage", &self.storage)
            .field("email_client", &self.email_client)
            .field("openai_client", &self.openai_client)
            .field("openai_assistant", &self.openai_assistant)
            .field("firebase_auth", &"<firebase_auth>")
            .finish()
    }
}
fn get_bearer_token(header: &str) -> Option<String> {
    let prefix_len = "Bearer ".len();

    match header.len() {
        l if l < prefix_len => None,
        _ => Some(header[prefix_len..].to_string()),
    }
}
#[derive(Debug, Clone)]
pub struct AppStatePtr {
    pub state: Arc<AppState>
}
#[async_trait]
impl FromRequestParts<AppStatePtr> for FirebaseUser {
    type Rejection = UnauthorizedResponse;

    async fn from_request_parts(parts: &mut Parts, app_state_ptr: &AppStatePtr) -> Result<Self, Self::Rejection> {
        let store = &app_state_ptr.state.firebase_auth;

        let auth_header = parts
            .headers
            .get(http::header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .unwrap_or("");

        let bearer = get_bearer_token(auth_header).map_or(
            Err(UnauthorizedResponse {
                msg: "Missing Bearer Token".to_string(),
            }),
            Ok,
        )?;

        match store.verify(&bearer) {
            Err(e) => {
                eprintln!("Failed to verify Token: {}", e);

                Err(UnauthorizedResponse {
                    msg: format!("Failed to verify Token: {}", e),
                })
            },
            Ok(current_user) => Ok(current_user),
        }
    }
}

pub struct UnauthorizedResponse {
    msg: String,
}

impl IntoResponse for UnauthorizedResponse {
    fn into_response(self) -> Response {
        (http::StatusCode::UNAUTHORIZED, self.msg).into_response()
    }
}
pub struct AppState {
    pub db: Mutex<Database>,
    pub storage: StorageClient,
    pub email_client: EmailClient,
    pub openai_client: Client<OpenAIConfig>,
    pub openai_assistant: Option<AssistantObject>,
    pub firebase_auth: FirebaseAuth
}
impl AppState {
    /// Initializes the application state with database, storage, and service clients.
    /// 
    /// # Returns
    /// * A successful result with the application state if successful
    /// 
    /// # Fails
    /// * If the database fails to initialize
    /// * If the storage client fails to initialize
    /// * If Firebase Auth fails to initialize
    /// * If the OpenAI assistant can't be loaded
    /// 
    /// # Notes
    /// * This function is typically called at the start of the application.
    /// * Required environment variables:
    ///   - `GOOGLE_APPLICATION_CREDENTIALS` - Path to GCP service account JSON
    ///   - `FIREBASE_ACCESS_KEY` - Firebase RTDB access key
    ///   - `OPENAI_ASSISTANT_ID` - OpenAI assistant ID
    ///   - AWS credentials for SES
    pub async fn new() -> Result<Self> {
        let client = Client::new();
        let firebase_auth = FirebaseAuth::new("network-technology-project")
            .await;

        // Try to initialize the assistant (optional for upload route)
        let assistant = match std::env::var("OPENAI_ASSISTANT_ID") {
            Ok(assistant_id) => {
                match client.assistants().retrieve(&assistant_id).await {
                    Ok(a) => {
                        println!("✅ OpenAI Assistant loaded successfully");
                        Some(a)
                    }
                    Err(e) => {
                        eprintln!("⚠️  Failed to load OpenAI Assistant: {}", e);
                        eprintln!("   Upload and processing will work, but AI assistant features will be disabled");
                        None
                    }
                }
            }
            Err(_) => {
                eprintln!("⚠️  OPENAI_ASSISTANT_ID not set");
                eprintln!("   Upload and processing will work, but AI assistant features will be disabled");
                None
            }
        };

        // Initialize Firebase Storage client
        let storage = StorageClient::new()
            .await
            .context("Failed to initialize Firebase Storage client")?;

        // Initialize email client
        let email_client = EmailClient::from_env()
            .await
            .context("Failed to initialize email client")?;

        Ok(Self {
            db: Mutex::new(Database::init().await.context("Failed to initialize database while setting up app state!")?),
            storage,
            email_client,
            openai_client: client,
            openai_assistant: assistant,
            firebase_auth
        })
    }
}


/// The error type for the application.
/// 
/// # Fields
/// * `AppError` - The error type for the application
/// 
/// # Notes
/// * This error type is used to handle errors in the application.
/// * The reason for its existence is to allow for a more detailed error message to be returned by `axum` routes.
#[derive(Debug)]
pub struct AppError(pub anyhow::Error);
impl IntoResponse for AppError {
    fn into_response(self) -> Response<Body> {
        let err = &self.0;

        eprintln!("Encountered an error: {err:#?}");
        for (ind, ctx) in err.chain().enumerate() {
            eprintln!("  [{ind}] {ctx:#?}");
        }

        eprintln!("Full backtrace...");
        eprintln!("{:#?}", err.backtrace());

        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}",err),
        )
            .into_response()
    }
}
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

