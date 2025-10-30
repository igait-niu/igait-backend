use std::path::PathBuf;

use super::{
    StageData, StageStatus,
    super::Output
};
use anyhow::{anyhow, ensure, Context, Result};
use tokio::process::Command;

// Use local assets directory instead of remote one
const PATH_TO_STAGE_6_ASSETS: &str = "./igait-pipeline/assets/stage_6";

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

    // Use bash to run module commands and Python setup
    // The module command is a bash function, so we need to invoke it through bash -l -c
    let setup_script = format!(
        r#"
        module purge && \
        module load python/python-3.12.4 && \
        python3 -m pip install -U pip --user && \
        pip3 install -r {} --user
        "#,
        output_requirements.display()
    );
    
    logs.push_str("Setting up Python environment with modules...\n");
    let output = Command::new("bash")
        .arg("-l")
        .arg("-c")
        .arg(&setup_script)
        .output()
        .await
        .context("Failed to run Python environment setup")?;
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    logs.push_str(&format!("Setup stdout:\n{stdout}\n"));
    logs.push_str(&format!("Setup stderr:\n{stderr}\n"));
    
    ensure!(output.status.success(), "Python environment setup failed");
    logs.push_str("Python environment setup complete.\n");

    // Run the Python script with the Model directory path from assets
    let run_script = format!(
        r#"
        module purge && \
        module load python/python-3.12.4 && \
        cd {} && \
        python3 {} {} {} {}
        "#,
        prediction_output_dir.display(),
        output_python_script.display(),
        prediction_output_dir.display(),
        prediction_output_dir.display(),
        model_dir_path.display()
    );
    
    logs.push_str("Running Python inference script...\n");
    let output = Command::new("bash")
        .arg("-l")
        .arg("-c")
        .arg(&run_script)
        .output()
        .await
        .context("Failed to run Python script")?;
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    logs.push_str(&format!("Python script stdout:\n{stdout}\n"));
    logs.push_str(&format!("Python script stderr:\n{stderr}\n"));
    
    ensure!(output.status.success(), "Running Python script failed");
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