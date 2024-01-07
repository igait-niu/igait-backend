#[derive(Debug)]
pub enum Status {
    Submitting,
    Submitted,
    SubmissionErr,
    Queue,
    Processing,
    ProcessingErr,
    Complete(f32)
}
#[derive(Debug)]
pub struct Request {
    pub id: usize,
    pub status: Status
}