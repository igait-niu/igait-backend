use std::process::Command;
use std::fs;
use crate::request::StatusCode;

pub async fn run_inference(id: usize) -> Result<StatusCode, String> {
    // Grab output from inference
    let output = Command::new("python")
        .arg("data/run_inference.py")
        .arg(id.to_string())
        .output()
        .map_err(|_| String::from("Failed to get output from model!") )?
        .stdout;
    
    // Build the stdout into a string
    let output_string = String::from_utf8(output.clone())
        .map_err(|_| format!("Invalid UTF-8 '{:?}' was produced from model!", output) )?;

    // Parse the confidence rating
    let confidence = output_string
        .parse::<f32>()
        .map_err(|_| format!("Could not parse the output to f32! Output: {}", output_string) )?;

    // Remove file
    if fs::remove_file(format!("data/queue/{}.mp4", id)).is_err() {
        println!("FAILED TO REMOVE 'data/queue/{}.mp4'!", id);
    };

    Ok(StatusCode::Complete(confidence))
}