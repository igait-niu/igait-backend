//! Stage 2: Validity Check Microservice
//! 
//! Verifies that a person can be detected in the uploaded videos.
//! 
//! NOTE: This is currently a placeholder that passes through immediately.

use anyhow::Result;
use async_trait::async_trait;
use igait_lib::microservice::{
    StageJobRequest, StageJobResult, StageNumber, StageProcessor, StageServer,
};
use std::collections::HashMap;
use std::time::Instant;

/// The validity check processor.
pub struct ValidityCheckProcessor;

#[async_trait]
impl StageProcessor for ValidityCheckProcessor {
    fn stage(&self) -> StageNumber {
        StageNumber::Stage2ValidityCheck
    }

    fn service_name(&self) -> &'static str {
        "igait-stage2-validity-check"
    }

    async fn process(&self, request: StageJobRequest) -> StageJobResult {
        let start_time = Instant::now();
        let mut logs = String::new();

        println!("Processing job {}: Validity Check (pass-through)", request.job_id);
        logs.push_str(&format!("Starting validity check for job {}\n", request.job_id));

        // Get input paths (from stage 1)
        let front_input = request.input_front_video();
        let side_input = request.input_side_video();

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

        StageJobResult::success(
            request.job_id,
            StageNumber::Stage2ValidityCheck,
            output_keys,
            logs,
            duration.as_millis() as u64,
        )
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);

    println!("Starting Stage 2 Validity Check service on port {}", port);

    StageServer::new(ValidityCheckProcessor)
        .port(port)
        .run()
        .await
}
