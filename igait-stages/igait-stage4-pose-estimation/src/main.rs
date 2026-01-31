//! Stage 4: Pose Estimation Microservice
//! 
//! Extracts body keypoints from videos using OpenPose or MediaPipe.
//! This is the most compute-intensive stage and may require GPU acceleration.

use anyhow::Result;
use async_trait::async_trait;
use igait_lib::microservice::{
    StageJobRequest, StageJobResult, StageNumber, StageProcessor, StageServer,
};
use std::collections::HashMap;
use std::time::Instant;

/// The pose estimation processor.
pub struct PoseEstimationProcessor;

#[async_trait]
impl StageProcessor for PoseEstimationProcessor {
    fn stage(&self) -> StageNumber {
        StageNumber::Stage4PoseEstimation
    }

    fn service_name(&self) -> &'static str {
        "igait-stage4-pose-estimation"
    }

    async fn process(&self, request: StageJobRequest) -> StageJobResult {
        let start_time = Instant::now();
        let mut logs = String::new();

        println!("Processing job {}: Pose Estimation", request.job_id);
        logs.push_str(&format!("Starting pose estimation for job {}\n", request.job_id));

        match self.do_pose_estimation(&request, &mut logs).await {
            Ok(output_keys) => {
                let duration = start_time.elapsed();
                logs.push_str(&format!("Pose estimation completed in {:?}\n", duration));
                
                StageJobResult::success(
                    request.job_id,
                    StageNumber::Stage4PoseEstimation,
                    output_keys,
                    logs,
                    duration.as_millis() as u64,
                )
            }
            Err(e) => {
                let duration = start_time.elapsed();
                eprintln!("Pose estimation failed for job {}: {}", request.job_id, e);
                logs.push_str(&format!("ERROR: {}\n", e));
                
                StageJobResult::failure(
                    request.job_id,
                    StageNumber::Stage4PoseEstimation,
                    e.to_string(),
                    logs,
                    duration.as_millis() as u64,
                )
            }
        }
    }
}

impl PoseEstimationProcessor {
    async fn do_pose_estimation(
        &self,
        request: &StageJobRequest,
        logs: &mut String,
    ) -> Result<HashMap<String, String>> {
        let front_input = request.input_keys.get("front_video")
            .ok_or_else(|| anyhow::anyhow!("Missing front_video input key"))?;
        let side_input = request.input_keys.get("side_video")
            .ok_or_else(|| anyhow::anyhow!("Missing side_video input key"))?;

        logs.push_str(&format!("Running pose estimation on front video: {}\n", front_input));
        logs.push_str(&format!("Running pose estimation on side video: {}\n", side_input));

        // TODO: Run OpenPose or MediaPipe
        // This would typically call a Python subprocess or use native bindings

        let stage_prefix = format!("jobs/{}/stage_4", request.job_id);
        let mut output_keys = HashMap::new();
        output_keys.insert("front_keypoints".to_string(), format!("{}/front_keypoints.json", stage_prefix));
        output_keys.insert("side_keypoints".to_string(), format!("{}/side_keypoints.json", stage_prefix));
        output_keys.insert("front_overlay".to_string(), format!("{}/front_overlay.mp4", stage_prefix));
        output_keys.insert("side_overlay".to_string(), format!("{}/side_overlay.mp4", stage_prefix));

        logs.push_str("Pose estimation complete (placeholder)\n");
        
        Ok(output_keys)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);

    println!("Starting Stage 4 Pose Estimation service on port {}", port);

    StageServer::new(PoseEstimationProcessor)
        .port(port)
        .run()
        .await
}
