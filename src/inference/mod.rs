use rand::random;
use std::process::Command;

pub async fn run_inference(id: usize) -> f32 {
    let output = Command::new("python")
        .arg("data/run_inference.py")
        .output().expect("todo!")
        .stdout;
    
    let output_string = String::from_utf8(output)
        .expect("todo!");

    println!(":3 -{}-", output_string);
        
    output_string
        .parse::<f32>()
        .unwrap()
}