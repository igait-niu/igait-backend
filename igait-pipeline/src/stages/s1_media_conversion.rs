use std::path::PathBuf;

use super::{
    StageData, StageStatus,
    super::Output
};
use anyhow::{Context, Result};
use tokio::process::Command;

const FFMPEG_PATH: &str = "/lstr/sahara/zwlab/ffmpeg/bin/ffmpeg";

pub async fn execute(
    output: &mut Output
) -> StageData {
    let mut logs = String::new();

    let status = match media_conversion(output, &mut logs).await {
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
async fn media_conversion (
    output: &mut Output,
    logs: &mut String
) -> Result<StageStatus> {
    let input_front_path = &output.canonical_paths.front_video;
    let input_side_path = &output.canonical_paths.side_video;

    // Create the "converted" subdirectory if it doesn't exist
    tokio::fs::create_dir_all(
        &output.canonical_paths.stage_paths.s1_media_conversion
    ).await
        .context("Failed to create output/converted directory")?;

    let output_front_path = &output.canonical_paths.stage_paths.s1_media_conversion.join("front.mp4");
    let output_side_path = &output.canonical_paths.stage_paths.s1_media_conversion.join("side.mp4");
    
    logs.push_str(&format!("Converting front video\n"));
    standardize_video(input_front_path, output_front_path, logs).await
        .context("Failed to convert front video!")?;
    logs.push_str("Front video conversion done.\n");

    logs.push_str(&format!("Converting side video\n"));
    standardize_video(input_side_path, output_side_path, logs).await
        .context("Failed to convert side video!")?;
    logs.push_str("Side video conversion done.\n");

    Ok(StageStatus::Done)
}
async fn standardize_video(
    input_file_path: &PathBuf,
    output_file_path: &PathBuf,
    logs: &mut String
) -> Result<()> {
    let output = Command::new(FFMPEG_PATH)
        .args([
            "-y", // Overwrite without asking
            "-i", input_file_path.to_str().context("Invalid input path!")?,
            "-vf", "scale=1920:1080:force_original_aspect_ratio=decrease,pad=1920:1080:(ow-iw)/2:(oh-ih)/2",
            "-r", "60",
            "-b:v", "5000k",
            "-maxrate", "5000k",
            "-bufsize", "10000k",
            "-preset", "fast", // x264 presets: ultrafast, superfast, veryfast, faster, fast, medium, slow, slower, veryslow
            "-c:v", "libx264", // Use software x264 encoder
            "-pix_fmt", "yuv420p",
            "-c:a", "aac",
            "-b:a", "192k",
            output_file_path.to_str().context("Invalid output path!")?,
        ])
        .output()
        .await
        .context("failed to run ffmpeg")?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    logs.push_str(&format!("ffmpeg stdout: {}\n", stdout));
    logs.push_str(&format!("ffmpeg stderr: {}\n", stderr));

    if !output.status.success() {
        return Err(anyhow::anyhow!("ffmpeg failed with status: {}!", output.status));
    }

    Ok(())
}