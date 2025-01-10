use std::time::SystemTime;

use s3::{creds::Credentials, Bucket};
use anyhow::{ Result, Context };
use axum::{
    body::Body,
    response::{IntoResponse, Response}
};
use serde::{Deserialize, Serialize};

use super::database::Database;
use crate::print_be;

/// The unique identifier for a job task.
pub type JobTaskID = u128;

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
/// * `task_number` - The task number of the backend to keep track of requests
/// 
/// # Notes
/// * The task number is used to keep track of requests and is incremented with each request.
/// * This struct is typically wrapped in an `Arc<Mutex<>>` to allow for concurrent access.
#[derive(Debug)]
pub struct AppState {
    pub db: Database,
    pub bucket: Bucket,
    pub task_number: JobTaskID,
    pub aws_ses_client: aws_sdk_sesv2::Client
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

        Ok(Self {
            db: Database::init().await.context("Failed to initialize database while setting up app state!")?,
            bucket: Bucket::new(
                "igait-storage",
                "us-east-2".parse().context("Improper region!")?,
                Credentials::default().context("Couldn't unpack credentials! Make sure that you have set AWS credentials in your system environment.")?,
            ).context("Failed to initialize bucket!")?,
            task_number: 0,
            aws_ses_client: aws_sdk_sesv2::Client::new(&aws_config)
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
        print_be!(0, "Encountered an error: {self:#?}");
        print_be!(0, "Returning an internal server error response.");
        print_be!(0, "Please check the logs for more information.");

        print_be!(0, "Printing the error chain...");
        for (ind, cause) in self.0.chain().enumerate() {
            eprintln!("[{ind}] {cause:#?}");
        }

        print_be!(0, "Printing the backtrace...");
        eprintln!("{:#?}", self.0.backtrace());

        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
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

