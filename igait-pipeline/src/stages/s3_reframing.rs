use super::{
    StageData, StageStatus,
    super::Output
};
use anyhow::{Context, Result};

async fn reframe (
    output: &mut Output,
    logs: &mut String
) -> Result<StageStatus> {
    // Check if we're skipping to this stage or later
    let is_skipping = output.skip_to_stage.map_or(false, |skip| skip >= 3);
    
    if is_skipping {
        // When skipping, verify required files exist from stage 1
        let converted_front = output.canonical_paths.stage_paths.s1_media_conversion.join("front.mp4");
        let converted_side = output.canonical_paths.stage_paths.s1_media_conversion.join("side.mp4");
        
        if !tokio::fs::try_exists(&converted_front).await
            .context("Failed to check if converted front video exists")? {
            anyhow::bail!("Skipping to Stage 3 but required file from Stage 1 doesn't exist: {}", converted_front.display());
        }
        if !tokio::fs::try_exists(&converted_side).await
            .context("Failed to check if converted side video exists")? {
            anyhow::bail!("Skipping to Stage 3 but required file from Stage 1 doesn't exist: {}", converted_side.display());
        }
        logs.push_str("Skipping to Stage 3: Verified required files from Stage 1 exist\n");
    } else {
        // When not skipping, verify previous stages completed successfully
        output.stages.s1_media_conversion
            .as_ref()
            .context("Stage 1 (Media Conversion) data missing")?
            .status
            .as_ref()
            .map_err(|e| anyhow::anyhow!("Stage 1 (Media Conversion) failed: {e}"))?;
        
        output.stages.s2_validity_check
            .as_ref()
            .context("Stage 2 (Validity Check) data missing")?
            .status
            .as_ref()
            .map_err(|e| anyhow::anyhow!("Stage 2 (Validity Check) failed: {e}"))?;
    }
    
    Ok(StageStatus::Skipped)
}

pub async fn execute(
    output: &mut Output
) -> StageData {
    let mut logs = String::new();
    let status = match reframe(output, &mut logs).await {
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