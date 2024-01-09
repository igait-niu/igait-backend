use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Status {
    Submitting,
    SubmissionErr(String),
    Queue,
    Processing,
    InferenceErr(String),
    Complete(f32)
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub id: usize,
    pub status: Status
}