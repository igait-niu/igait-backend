//! Stage 4: Pose Estimation Microservice
//!
//! Extracts body keypoints from videos using OpenPose or MediaPipe.
//! This is the most compute-intensive stage and may require GPU acceleration.
//!
//! NOTE: This is currently a placeholder that passes through immediately.

use anyhow::Result;
use async_trait::async_trait;
use igait_lib::microservice::{
    run_stage_worker, ProcessingResult, QueueItem, StageNumber, StageWorker,
};
use std::collections::HashMap;
use std::time::Instant;

/// The pose estimation worker.
pub struct PoseEstimationWorker;

#[async_trait]
impl StageWorker for PoseEstimationWorker {
    fn stage(&self) -> StageNumber {
        StageNumber::Stage4PoseEstimation
    }

    fn service_name(&self) -> &'static str {
        "igait-stage4-pose-estimation"
    }

    async fn process(&self, job: &QueueItem) -> ProcessingResult {
        let start_time = Instant::now();
        let mut logs = String::new();

        println!(
            "Processing job {}: Pose Estimation (pass-through)",
            job.job_id
        );
        logs.push_str(&format!(
            "Starting pose estimation for job {}\n",
            job.job_id
        ));

        let stage = StageNumber::Stage4PoseEstimation;

        // Get input paths (from stage 3)
        let front_input = job.input_front_video(stage);
        let side_input = job.input_side_video(stage);

        logs.push_str(&format!("Input front video: {}\n", front_input));
        logs.push_str(&format!("Input side video: {}\n", side_input));

        // For now, just pass through - output the same paths as input
        // TODO: Implement actual pose estimation logic (OpenPose/MediaPipe)
        let mut output_keys = HashMap::new();
        output_keys.insert("front_video".to_string(), front_input);
        output_keys.insert("side_video".to_string(), side_input);
        // In the future, this should include pose keypoint data files
        // output_keys.insert("front_keypoints".to_string(), front_keypoints_path);
        // output_keys.insert("side_keypoints".to_string(), side_keypoints_path);

        logs.push_str("Pose estimation complete (placeholder - no actual estimation performed)\n");

        let duration = start_time.elapsed();
        logs.push_str(&format!("Completed in {:?}\n", duration));

        ProcessingResult::Success {
            output_keys,
            logs,
            duration_ms: duration.as_millis() as u64,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting Stage 4 Pose Estimation worker...");
    run_stage_worker(PoseEstimationWorker).await
}
