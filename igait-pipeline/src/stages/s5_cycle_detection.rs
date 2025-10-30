use igait_lib::{StageData, StageStatus, Output};
use anyhow::{Context, Result};

async fn detect_cycles (
    output: &mut Output,
    logs: &mut String
) -> Result<StageStatus> {
    // Check if we're skipping to this stage or later
    let is_skipping = output.skip_to_stage.map_or(false, |skip| skip >= 5);
    
    let pose_estimation_output_dir = &output.canonical_paths.stage_paths.s4_pose_estimation;
    let pose_estimation_json_dir = pose_estimation_output_dir.join("json");
    
    if is_skipping {
        // When skipping, verify required files exist from stage 4
        if !tokio::fs::try_exists(&pose_estimation_json_dir).await
            .context("Failed to check if pose estimation JSON directory exists")? {
            anyhow::bail!("Skipping to Stage 5 but required files from Stage 4 (Pose Estimation) don't exist at: {}", pose_estimation_json_dir.display());
        }
        logs.push_str("Skipping to Stage 5: Verified required files from Stage 4 exist\n");
    } else {
        // When not skipping, verify previous stage completed successfully
        let pose_estimation_stage = output.stages.s4_pose_estimation
            .as_ref()
            .context("Stage 4 (Pose Estimation) data missing")?
            .status
            .as_ref()
            .map_err(|e| anyhow::anyhow!("Stage 4 (Pose Estimation) failed: {e}"))?;
        anyhow::ensure!(matches!(pose_estimation_stage, StageStatus::Done), "Stage 4 (Pose Estimation) did not complete successfully");
    }
    
    Ok(StageStatus::Skipped)
}

pub async fn execute(
    output: &mut Output
) -> StageData {
    let mut logs = String::new();
    let status = match detect_cycles(output, &mut logs).await {
        Ok(status) => Ok(status),
        Err(e) => {
            let mut error_body = format!("{e:?}");
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