//! Stage 6: Prediction Microservice
//!
//! Runs the ML model to predict ASD classification from gait data.
//!
//! NOTE: This is currently a placeholder that passes through immediately.

use anyhow::Result;
use async_trait::async_trait;
use igait_lib::microservice::{
    run_stage_worker, ProcessingResult, QueueItem, StageNumber, StageWorker,
};
use std::collections::HashMap;
use std::time::Instant;

/// The prediction worker.
pub struct PredictionWorker;

#[async_trait]
impl StageWorker for PredictionWorker {
    fn stage(&self) -> StageNumber {
        StageNumber::Stage6Prediction
    }

    fn service_name(&self) -> &'static str {
        "igait-stage6-prediction"
    }

    async fn process(&self, job: &QueueItem) -> ProcessingResult {
        let start_time = Instant::now();
        let mut logs = String::new();

        println!("Processing job {}: Prediction (pass-through)", job.job_id);
        logs.push_str(&format!("Starting prediction for job {}\n", job.job_id));

        let stage = StageNumber::Stage6Prediction;

        // Get input paths (from stage 5)
        let front_input = job.input_front_video(stage);
        let side_input = job.input_side_video(stage);

        logs.push_str(&format!("Input front video: {}\n", front_input));
        logs.push_str(&format!("Input side video: {}\n", side_input));

        // For now, just pass through - output the same paths as input
        // TODO: Implement actual ML prediction logic
        let mut output_keys = HashMap::new();
        output_keys.insert("front_video".to_string(), front_input);
        output_keys.insert("side_video".to_string(), side_input);
        // In the future, this should include prediction results
        // output_keys.insert("prediction".to_string(), prediction_path);
        // output_keys.insert("score".to_string(), score_string);

        logs.push_str("Prediction complete (placeholder - no actual prediction performed)\n");

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
    println!("Starting Stage 6 Prediction worker...");
    run_stage_worker(PredictionWorker).await
}
