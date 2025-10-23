use std::{sync::Arc, time::SystemTime};

use s3::{creds::Credentials, Bucket};
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
use tracing::error;

use super::database::Database;

/// The user struct, which contains a user ID and a list of jobs.
/// 
/// # Fields
/// * `uid` - The user ID
/// * `jobs` - The list of jobs
#[derive( Serialize, Deserialize, Debug )]
pub struct User {
    pub uid: String,
    pub jobs: Vec<Job>
}

/// The job struct, which contains the job
/// 
/// # Fields
/// * `age` - The age of the patient
/// * `ethnicity` - The ethnicity of the patient
/// * `sex` - The assigned sex of the patient
/// * `height` - The height of the patient
/// * `status` - The status of the job
/// * `timestamp` - The timestamp of the job
/// * `weight` - The weight of the patient
/// * `email` - The email of the person who submitted the job
#[derive( Serialize, Deserialize, Clone, Debug )]
pub struct Job {
    pub age: i16,
    pub ethnicity: String,
    pub sex: char,
    pub height: String,
    pub status: JobStatus,
    pub timestamp: SystemTime,
    pub weight: i16,
    pub email: String
}

/// The job status struct, which contains the status of the job.
/// 
/// # Fields
/// * `code` - The status code of the job
/// * `value` - The human-readable value of the status
#[derive( Serialize, Deserialize, Clone, Debug )]
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
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum JobStatusCode {
    Submitting,
    SubmissionErr,
    Queue,
    Processing,
    InferenceErr,
    Complete
}

/// The request struct for the historical submissions endpoint.
/// 
/// # Fields
/// * `id` - The ID of the request
/// * `status` - The status of the request
#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub id: usize,
    pub status: JobStatusCode
}

/// The state of the entire backend application with handles to the database and S3 bucket.
/// 
/// # Fields
/// * `db` - The database handle
/// * `bucket` - The S3 bucket handle
/// 
/// # Notes
/// * The task number is used to keep track of requests and is incremented with each request.
/// * This struct is typically wrapped in an `Arc<Mutex<>>` to allow for concurrent access.
impl std::fmt::Debug for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppState")
            .field("db", &self.db)
            .field("bucket", &self.bucket)
            .field("aws_ses_client", &self.aws_ses_client)
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
                error!("Failed to verify Token: {}", e);

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
    pub bucket: Mutex<Bucket>,
    pub aws_ses_client: Mutex<aws_sdk_sesv2::Client>,
    pub openai_client: Client<OpenAIConfig>,
    pub openai_assistant: AssistantObject,
    pub firebase_auth: FirebaseAuth
}
impl AppState {
    /// Initializes the application state with a new database and S3 bucket.
    /// 
    /// # Returns
    /// * A successful result with the application state if successful
    /// 
    /// # Fails
    /// * If the database fails to initialize
    /// * If the S3 bucket fails to initialize
    /// * If the credentials can't be unpacked
    /// 
    /// # Notes
    /// * This function is typically called at the start of the application to initialize the state.
    /// * The environment variables `AWS_ACCESS_KEY_ID` and `AWS_SECRET_ACCESS_KEY` must be set.
    pub async fn new() -> Result<Self> {
        let aws_config = aws_config::load_from_env().await;
        let client = Client::new();
        let firebase_auth = FirebaseAuth::new("network-technology-project")
            .await;

        // Initialize the assistant
        let assistant_id = std::env::var("OPENAI_ASSISTANT_ID")
            .context("Couldn't find the OpenAI assistant ID!")?;
        let assistant = client
            .assistants()
            .retrieve(&assistant_id)
            .await
            .context("Failed to retrieve assistant")?;

        Ok(Self {
            db: Mutex::new(Database::init().await.context("Failed to initialize database while setting up app state!")?),
            bucket: Mutex::new(Bucket::new(
                "igait-storage",
                "us-east-2".parse().context("Improper region!")?,
                Credentials::default().context("Couldn't unpack credentials! Make sure that you have set AWS credentials in your system environment.")?,
            ).context("Failed to initialize bucket!")?),
            aws_ses_client: Mutex::new(aws_sdk_sesv2::Client::new(&aws_config)),
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
    #[tracing::instrument]
    fn into_response(self) -> Response<Body> {
        let err = &self.0;

        error!("Encountered an error: {err:#?}");
        for (ind, ctx) in err.chain().enumerate() {
            error!("  [{ind}] {ctx:#?}");
        }

        error!("Full backtrace...");
        error!("{:#?}", err.backtrace());

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

