use std::process::Command;
use tokio::fs::remove_dir_all;

pub async fn run_inference(dir_id: String, front_file_ext: String, side_file_ext: String) -> Result<f32, String> {
    // Grab output from inference
    let output = Command::new("python")
        .arg("data/run_inference.py")
        .arg(format!("{dir_id}/front.{front_file_ext}"))
        .arg(format!("{dir_id}/side.{side_file_ext}"))
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
    if remove_dir_all(format!("data/queue/{}", dir_id)).await.is_err() {
        println!("FAILED TO REMOVE 'data/queue/{}'!", dir_id);
    };

    Ok(confidence)
}