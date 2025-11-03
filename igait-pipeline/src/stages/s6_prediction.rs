use std::path::PathBuf;

use igait_lib::{StageData, StageStatus, Output};
use anyhow::{anyhow, ensure, Context, Result};
use tokio::process::Command;

// Workspace root is guaranteed to be at this location
const PATH_TO_STAGE_6_ASSETS: &str = "/lstr/sahara/zwlab/jw/igait-pipeline/igait-pipeline/assets/stage_6";

async fn predict (
    output: &mut Output,
    logs: &mut String
) -> Result<StageStatus> {
    // Check if we're skipping to this stage or later
    let is_skipping = output.skip_to_stage.map_or(false, |skip| skip >= 6);
    
    let pose_estimation_output_dir = &output.canonical_paths.stage_paths.s4_pose_estimation;
    let pose_estimation_json_dir = pose_estimation_output_dir.join("json");
    
    if is_skipping {
        // When skipping, verify required files exist from stage 4
        if !tokio::fs::try_exists(&pose_estimation_json_dir).await
            .context("Failed to check if pose estimation JSON directory exists")? {
            anyhow::bail!("Skipping to Stage 6 but required files from Stage 4 (Pose Estimation) don't exist at: {}", pose_estimation_json_dir.display());
        }
        logs.push_str("Skipping to Stage 6: Verified required files from Stage 4 exist\n");
    } else {
        // When not skipping, verify previous stage completed successfully
        let pose_estimation_stage = output.stages.s4_pose_estimation
            .as_ref()
            .context("Stage 4 (Pose Estimation) data missing")?
            .status
            .as_ref()
            .map_err(|e| anyhow!("Stage 4 (Pose Estimation) failed: {e}"))?;
        ensure!(matches!(pose_estimation_stage, StageStatus::Done), "Stage 4 (Pose Estimation) did not complete successfully");
    }
    
    let prediction_output_dir = &output.canonical_paths.stage_paths.s6_prediction;

    // Create the output subdirectory if it doesn't exist
    tokio::fs::create_dir_all(prediction_output_dir).await
        .context("Failed to create output/s6_prediction directory")?;

    // Copy JSON directory using cp command (since tokio::fs::copy doesn't work for directories)
    let copy_output = Command::new("cp")
        .arg("-r")
        .arg(&pose_estimation_json_dir)
        .arg(prediction_output_dir)
        .output()
        .await
        .context("Failed to execute cp command for JSON directory")?;
    ensure!(copy_output.status.success(), "Failed to copy JSON files from Stage 4 (Pose Estimation) to Stage 6 (Prediction)");

    // Build paths for all files necessary for inference
    let assets_dir_path = &PathBuf::from(PATH_TO_STAGE_6_ASSETS);
    let model_dir_path = &assets_dir_path.join("Model").canonicalize()
        .context("Failed to canonicalize Model directory path")?;
    let python_script_path = &assets_dir_path.join("main.py");
    let requirements_path = &assets_dir_path.join("requirements.txt");

    // Copy only the Python script and requirements to the output directory
    let output_python_script = &prediction_output_dir.join("main.py");
    tokio::fs::copy(python_script_path, output_python_script).await
        .context("Failed to copy main.py to output/s6_prediction/main.py")?;
    let output_requirements = &prediction_output_dir.join("requirements.txt");
    tokio::fs::copy(requirements_path, output_requirements).await
        .context("Failed to copy requirements.txt to output/s6_prediction/requirements.txt")?;

    // Install Python dependencies directly (pip should be in PATH from PBS environment)
    logs.push_str("Installing Python dependencies...\n");
    let pip_output = Command::new("pip3.12")
        .arg("install")
        .arg("-r")
        .arg(output_requirements.as_os_str())
        .arg("--user")
        .output()
        .await
        .context("Failed to run pip3.12 install")?;
    
    let stdout = String::from_utf8_lossy(&pip_output.stdout);
    let stderr = String::from_utf8_lossy(&pip_output.stderr);
    logs.push_str(&format!("pip3 stdout:\n{stdout}\n"));
    logs.push_str(&format!("pip3 stderr:\n{stderr}\n"));
    
    ensure!(pip_output.status.success(), "pip3 install failed");
    logs.push_str("Python dependencies installed.\n");

    // Run the Python script with python3.12 explicitly (matching the pip version)
    logs.push_str("Running Python inference script...\n");
    
    // Set PYTHONPATH to include user site-packages directory
    let home_dir = std::env::var("HOME").unwrap_or_else(|_| "/home/z1994244".to_string());
    let python_user_site = format!("{home_dir}/.local/lib/python3.12/site-packages");
    
    let python_output = Command::new("python3.12")
        .arg(output_python_script.as_os_str())
        .arg(prediction_output_dir.as_os_str())
        .arg(prediction_output_dir.as_os_str())
        .arg(model_dir_path.as_os_str())
        .current_dir(prediction_output_dir)
        .env("PYTHONPATH", &python_user_site)
        .output()
        .await
        .context("Failed to run Python script")?;
    
    let stdout = String::from_utf8_lossy(&python_output.stdout);
    let stderr = String::from_utf8_lossy(&python_output.stderr);
    logs.push_str(&format!("Python script stdout:\n{stdout}\n"));
    logs.push_str(&format!("Python script stderr:\n{stderr}\n"));
    
    ensure!(python_output.status.success(), "Running Python script failed");
    logs.push_str("Python script completed successfully.\n");

    Ok(StageStatus::Done)
}
pub async fn execute(
    output: &mut Output
) -> StageData {
    let mut logs = String::new();

    let status = match predict(output, &mut logs).await {
        Ok(status) => Ok(status),
        Err(e) => {
            let mut error_body = format!("{e:?}\n");

            for (ind, err) in e.chain().enumerate() {
                error_body.push_str(&format!("{ind}: {err}\n"));
            }

            Err(error_body)
        }
    };

    StageData {
        status,
        logs,
    }
}