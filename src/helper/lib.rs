use std::time::SystemTime;

use s3::{creds::Credentials, Bucket};
use anyhow::{ Result, Context };
use axum::{
    body::Body,
    response::{IntoResponse, Response}
};
use serde::{Deserialize, Serialize};

use super::database::Database;


pub type JobTaskID = u128;
#[derive( Serialize, Deserialize, Debug )]
pub struct User {
    pub uid: String,
    pub jobs: Vec<Job>
}
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
#[derive( Serialize, Deserialize, Clone, Debug )]
pub struct JobStatus {
    pub code: JobStatusCode,
    pub value: String,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum JobStatusCode {
    Submitting,
    SubmissionErr,
    Queue,
    Processing,
    InferenceErr,
    Complete
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub id: usize,
    pub status: JobStatusCode
}
#[derive(Debug)]
pub struct AppState {
    pub db: Database,
    pub bucket: Bucket,
    pub task_number: JobTaskID
}
impl AppState {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            db: Database::init().await.context("Failed to initialize database while setting up app state!")?,
            bucket: Bucket::new(
                "igait-storage",
                "us-east-2".parse().context("Improper region!")?,
                Credentials::default().context("Couldn't unpack credentials! Make sure that you have set AWS credentials in your system environment.")?,
            ).context("Failed to initialize bucket!")?,
            task_number: 0
        })
    }
}


/* 
    The purpose of this interface is to allow our routes to use anyhow's 
     error handling system to return errors in a way that can be easily
     converted into a response. This is done by implementing the IntoResponse
     trait for the AppError struct, which is a wrapper around anyhow::Error.
 */
#[derive(Debug)]
pub struct AppError(pub anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response<Body> {
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