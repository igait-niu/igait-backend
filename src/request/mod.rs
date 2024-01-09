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
impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::SubmissionErr(err) | Status::InferenceErr(err) => {
                write!(f, r#"{{ "type": "Error", "value": "{}"}}"#, err)
            },
            Status::Complete(conf) => {
                write!(f, r#"{{ "type": "Complete", "value": "{}"}}"#, conf)
            },
            _ => write!(f, r#"{{ "type": "{:?}"}}"#, self)
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub id: usize,
    pub status: Status
}