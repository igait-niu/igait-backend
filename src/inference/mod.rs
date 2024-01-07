use std::process::Command;
use crate::request::Status;

pub async fn run_inference(id: usize) -> Result<Status, String> {
    let output = Command::new("python")
        .arg("data/run_inference.py")
        .arg(id.to_string())
        .output()
        .map_err(|_| String::from("Failed to get output from model!") )?
        .stdout;
    
    let output_string = String::from_utf8(output.clone())
        .map_err(|_| format!("Invalid UTF-8 '{:?}' was produced from model!", output) )?;

        
    let confidence = output_string
        .parse::<f32>()
        .map_err(|_| format!("Could not parse {} to f32!", output_string) )?;

    Ok(Status::Complete(confidence))
}