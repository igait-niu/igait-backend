#[derive(Debug)]
pub enum Status {
    Submitting,
    SubmissionErr(String),
    Queue,
    Processing,
    InferenceErr(String),
    Complete(f32)
}
#[derive(Debug)]
pub struct Request {
    pub id: usize,
    pub status: Status
}