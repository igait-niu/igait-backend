use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum StatusCode {
    Submitting,
    SubmissionErr(String),
    Queue,
    Processing,
    InferenceErr(String),
    Complete(f32)
}
impl std::fmt::Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StatusCode::SubmissionErr(err) | StatusCode::InferenceErr(err) => {
                write!(f, r#"{{ "type": "Error", "value": "{}"}}"#, err)
            },
            StatusCode::Complete(conf) => {
                write!(f, r#"{{ "type": "Complete", "value": "{}"}}"#, conf)
            },
            _ => write!(f, r#"{{ "type": "{:?}"}}"#, self)
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub id: usize,
    pub status: StatusCode
}