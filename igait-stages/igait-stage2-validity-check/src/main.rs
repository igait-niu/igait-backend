//! Stage 2: Validity Check Microservice
//!
//! Verifies that a person can be detected in the uploaded videos.
//!
//! NOTE: This is currently a placeholder that passes through immediately.

use anyhow::Result;
use async_trait::async_trait;
use igait_lib::microservice::{
    run_stage_worker, ProcessingResult, QueueItem, StageNumber, StageWorker,
};
use std::collections::HashMap;
use std::time::Instant;

/// The validity check worker.
pub struct ValidityCheckWorker;

#[async_trait]
impl StageWorker for ValidityCheckWorker {
    fn stage(&self) -> StageNumber {
        StageNumber::Stage2ValidityCheck
    }

    fn service_name(&self) -> &'static str {
        "igait-stage2-validity-check"
    }

    async fn process(&self, job: &QueueItem) -> ProcessingResult {
        let start_time = Instant::now();
        let mut logs = String::new();

        println!(
            "Processing job {}: Validity Check (pass-through)",
            job.job_id
        );
        logs.push_str(&format!(
            "Starting validity check for job {}\n",
            job.job_id
        ));

        let stage = StageNumber::Stage2ValidityCheck;

        // Get input paths (from stage 1)
        let front_input = job.input_front_video(stage);
        let side_input = job.input_side_video(stage);

        logs.push_str(&format!("Input front video: {}\n", front_input));
        logs.push_str(&format!("Input side video: {}\n", side_input));

        // For now, just pass through - output the same paths as input
        // (Stage 2 doesn't modify videos, just validates them)
        let mut output_keys = HashMap::new();
        output_keys.insert("front_video".to_string(), front_input);
        output_keys.insert("side_video".to_string(), side_input);

        logs.push_str("Validity check complete (placeholder - no actual validation performed)\n");

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
    println!("Starting Stage 2 Validity Check worker...");
    run_stage_worker(ValidityCheckWorker).await
}
