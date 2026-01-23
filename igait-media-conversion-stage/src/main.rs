//! Stage 1: Media Conversion Microservice
//! 
//! Converts uploaded videos to a standardized format:
//! - Resolution: 1920x1080 (padded if needed)
//! - Frame rate: 60fps
//! - Codec: H.264 (libx264)
//! - Audio: AAC 192kbps

use anyhow::Result;
use async_trait::async_trait;
use igait_lib::microservice::{
    StageJobRequest, StageJobResult, StageNumber, StageProcessor, StageServer,
};
use std::collections::HashMap;
use std::time::Instant;

/// The media conversion processor.
pub struct MediaConversionProcessor;

#[async_trait]
impl StageProcessor for MediaConversionProcessor {
    fn stage(&self) -> StageNumber {
        StageNumber::Stage1MediaConversion
    }

    fn service_name(&self) -> &'static str {
        "igait-stage1-media-conversion"
    }

    async fn process(&self, request: StageJobRequest) -> StageJobResult {
        let start_time = Instant::now();
        let mut logs = String::new();

        println!("Processing job {}: Media Conversion", request.job_id);
        logs.push_str(&format!("Starting media conversion for job {}\n", request.job_id));

        // TODO: Implement actual processing:
        // 1. Download input files from Firebase Storage
        // 2. Run FFmpeg conversion
        // 3. Upload output files to Firebase Storage

        match self.do_conversion(&request, &mut logs).await {
            Ok(output_keys) => {
                let duration = start_time.elapsed();
                logs.push_str(&format!("Conversion completed in {:?}\n", duration));
                
                StageJobResult::success(
                    request.job_id,
                    StageNumber::Stage1MediaConversion,
                    output_keys,
                    logs,
                    duration.as_millis() as u64,
                )
            }
            Err(e) => {
                let duration = start_time.elapsed();
                eprintln!("Conversion failed for job {}: {}", request.job_id, e);
                logs.push_str(&format!("ERROR: {}\n", e));
                
                StageJobResult::failure(
                    request.job_id,
                    StageNumber::Stage1MediaConversion,
                    e.to_string(),
                    logs,
                    duration.as_millis() as u64,
                )
            }
        }
    }
}

impl MediaConversionProcessor {
    async fn do_conversion(
        &self,
        request: &StageJobRequest,
        logs: &mut String,
    ) -> Result<HashMap<String, String>> {
        // Get input file paths from request
        let front_input = request.input_keys.get("front_video")
            .ok_or_else(|| anyhow::anyhow!("Missing front_video input key"))?;
        let side_input = request.input_keys.get("side_video")
            .ok_or_else(|| anyhow::anyhow!("Missing side_video input key"))?;

        logs.push_str(&format!("Input front: {}\n", front_input));
        logs.push_str(&format!("Input side: {}\n", side_input));

        // TODO: Download files from storage
        // TODO: Run FFmpeg conversion
        // TODO: Upload converted files

        // For now, return placeholder output keys
        let stage_prefix = format!("jobs/{}/stage_1", request.job_id);
        let mut output_keys = HashMap::new();
        output_keys.insert("front_video".to_string(), format!("{}/front.mp4", stage_prefix));
        output_keys.insert("side_video".to_string(), format!("{}/side.mp4", stage_prefix));

        logs.push_str("Conversion complete (placeholder)\n");
        
        Ok(output_keys)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);

    println!("Starting Stage 1 Media Conversion service on port {}", port);

    StageServer::new(MediaConversionProcessor)
        .port(port)
        .run()
        .await
}
