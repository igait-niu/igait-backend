use rand::random;
use std::process::Command;
use crate::request::Status;

pub async fn run_inference(id: usize) -> Result<Status, ()> {
    let output = Command::new("python")
        .arg("data/run_inference.py")
        .output()
        .map_err(|_| () )?
        .stdout;
    
    let output_string = String::from_utf8(output)
        .map_err(|_| () )?;

        
    let confidence = output_string
        .parse::<f32>()
        .map_err(|_| () )?;

    Ok(Status::Complete(confidence))
}