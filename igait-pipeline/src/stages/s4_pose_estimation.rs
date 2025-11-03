use std::path::PathBuf;

use igait_lib::{StageData, StageStatus, Output};
use anyhow::{Result, Context, anyhow};
use tokio::process::Command;

const PATH_TO_OPENPOSE_SIF: &str = "/lstr/sahara/zwlab/jw/igait-pipeline/igait-openpose/igait-openpose.sif";

enum CameraView {
    Front,
    Side,
}

pub async fn execute(
    output: &mut Output
) -> StageData {
    let mut logs = String::new();

    let status = match pose_estimation(output, &mut logs).await {
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

async fn pose_estimation (
    output: &mut Output,
    logs: &mut String
) -> Result<StageStatus> {
    // Check if we're skipping to this stage or later
    let is_skipping = output.skip_to_stage.map_or(false, |skip| skip >= 4);
    
    if is_skipping {
        // When skipping, verify required files exist from stage 1
        let converted_front = output.canonical_paths.stage_paths.s1_media_conversion.join("front.mp4");
        let converted_side = output.canonical_paths.stage_paths.s1_media_conversion.join("side.mp4");
        
        if !tokio::fs::try_exists(&converted_front).await
            .context("Failed to check if converted front video exists")? {
            anyhow::bail!("Skipping to Stage 4 but required file from Stage 1 doesn't exist: {}", converted_front.display());
        }
        if !tokio::fs::try_exists(&converted_side).await
            .context("Failed to check if converted side video exists")? {
            anyhow::bail!("Skipping to Stage 4 but required file from Stage 1 doesn't exist: {}", converted_side.display());
        }
        logs.push_str("Skipping to Stage 4: Verified required files from Stage 1 exist\n");
    } else {
        // When not skipping, verify previous stage completed successfully
        output.stages.s1_media_conversion
            .as_ref()
            .context("Stage 1 (Media Conversion) data missing")?
            .status
            .as_ref()
            .map_err(|e| anyhow!("Stage 1 (Media Conversion) failed: {e}"))?;
    }

    let output_dir = &output.canonical_paths.output_dir;
    let converted_stage_path = &output.canonical_paths.stage_paths.s1_media_conversion;
    let converted_stage_dir_name = converted_stage_path.file_name()
        .and_then(|os_str| os_str.to_str())
        .ok_or(anyhow!("Failed to get converted directory name"))?;
    let stage_output_dir = &output.canonical_paths.stage_paths.s4_pose_estimation;
    let stage_output_dir_name = stage_output_dir.file_name()
        .and_then(|os_str| os_str.to_str())
        .ok_or(anyhow!("Failed to get output directory name"))?;
    let stage_output_json_dir = &stage_output_dir.join("json");
    let stage_output_videos_dir = &stage_output_dir.join("videos");

    // Create output subdirectories if they don't exist
    tokio::fs::create_dir_all(stage_output_json_dir).await
        .context("Failed to create output json directory")?;
    tokio::fs::create_dir_all(stage_output_videos_dir).await
        .context("Failed to create output videos directory")?;

    logs.push_str("Running OpenPose on front video...\n");
    run_openpose(
        output_dir,
        stage_output_dir_name,
        converted_stage_dir_name,
        CameraView::Front,
        logs
    ).await.context("Failed to run OpenPose on front video")?;
    logs.push_str("OpenPose on front video done.\n");
    logs.push_str("Running OpenPose on side video...\n");
    run_openpose(
        output_dir,
        stage_output_dir_name,
        converted_stage_dir_name,
        CameraView::Side,
        logs
    ).await.context("Failed to run OpenPose on side video")?;
    logs.push_str("OpenPose on side video done.\n");

    Ok(StageStatus::Done)
}

async fn run_openpose(
    output_dir:  &PathBuf,
    stage_output_dir_name: &str,
    converted_stage_dir_name: &str,
    camera_view: CameraView,
    logs:        &mut String
) -> Result<()> {
    let view = match camera_view {
        CameraView::Front => "front",
        CameraView::Side  => "side",
    };
    let filename = format!("{view}.mp4");
    
    let output = Command::new("singularity")
        .arg("exec")
        .arg("--nv")
        .arg("--pwd").arg("/openpose")
        .arg("--bind")
        .arg(format!("{}:/outputs", output_dir.to_string_lossy()))
        .arg(PATH_TO_OPENPOSE_SIF)
        .arg("./build/examples/openpose/openpose.bin")
        .arg("--video").arg(format!("/outputs/{converted_stage_dir_name}/{filename}"))
        .arg("--display").arg("0")
        .arg("--write_json").arg(format!("/outputs/{stage_output_dir_name}/json/{view}"))
        .arg("--write_video").arg(format!("/outputs/{stage_output_dir_name}/videos/{filename}"))
        .output()
        .await
        .context("Failed to execute singularity command")?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    logs.push_str(&format!("OpenPose stdout:\n{stdout}\n"));
    logs.push_str(&format!("OpenPose stderr:\n{stderr}\n"));
    if !output.status.success() {
        return Err(anyhow::anyhow!("OpenPose command failed with exit code: {:?}", output.status.code()));
    }
    Ok(())
}