use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum StatusCode {
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
    pub status: StatusCode
}