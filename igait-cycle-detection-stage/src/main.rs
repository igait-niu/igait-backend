//! Stage 5: Cycle Detection Microservice
//! 
//! Analyzes pose keypoint data to identify individual gait cycles.

use anyhow::Result;
use async_trait::async_trait;
use igait_lib::microservice::{
    StageJobRequest, StageJobResult, StageNumber, StageProcessor, StageServer,
};
use std::collections::HashMap;
use std::time::Instant;

/// The cycle detection processor.
pub struct CycleDetectionProcessor;

#[async_trait]
impl StageProcessor for CycleDetectionProcessor {
    fn stage(&self) -> StageNumber {
        StageNumber::Stage5CycleDetection
    }

    fn service_name(&self) -> &'static str {
        "igait-stage5-cycle-detection"
    }

    async fn process(&self, request: StageJobRequest) -> StageJobResult {
        let start_time = Instant::now();
        let mut logs = String::new();

        println!("Processing job {}: Cycle Detection", request.job_id);
        logs.push_str(&format!("Starting cycle detection for job {}\n", request.job_id));

        match self.do_cycle_detection(&request, &mut logs).await {
            Ok(output_keys) => {
                let duration = start_time.elapsed();
                logs.push_str(&format!("Cycle detection completed in {:?}\n", duration));
                
                StageJobResult::success(
                    request.job_id,
                    StageNumber::Stage5CycleDetection,
                    output_keys,
                    logs,
                    duration.as_millis() as u64,
                )
            }
            Err(e) => {
                let duration = start_time.elapsed();
                eprintln!("Cycle detection failed for job {}: {}", request.job_id, e);
                logs.push_str(&format!("ERROR: {}\n", e));
                
                StageJobResult::failure(
                    request.job_id,
                    StageNumber::Stage5CycleDetection,
                    e.to_string(),
                    logs,
                    duration.as_millis() as u64,
                )
            }
        }
    }
}

impl CycleDetectionProcessor {
    async fn do_cycle_detection(
        &self,
        request: &StageJobRequest,
        logs: &mut String,
    ) -> Result<HashMap<String, String>> {
        let front_keypoints = request.input_keys.get("front_keypoints")
            .ok_or_else(|| anyhow::anyhow!("Missing front_keypoints input key"))?;
        let side_keypoints = request.input_keys.get("side_keypoints")
            .ok_or_else(|| anyhow::anyhow!("Missing side_keypoints input key"))?;

        logs.push_str(&format!("Analyzing front keypoints: {}\n", front_keypoints));
        logs.push_str(&format!("Analyzing side keypoints: {}\n", side_keypoints));

        // TODO: Implement cycle detection algorithm

        let stage_prefix = format!("jobs/{}/stage_5", request.job_id);
        let mut output_keys = HashMap::new();
        output_keys.insert("cycles".to_string(), format!("{}/cycles.json", stage_prefix));
        // Pass through keypoints for next stage
        output_keys.insert("front_keypoints".to_string(), front_keypoints.clone());
        output_keys.insert("side_keypoints".to_string(), side_keypoints.clone());

        logs.push_str("Cycle detection complete (placeholder)\n");
        
        Ok(output_keys)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);

    println!("Starting Stage 5 Cycle Detection service on port {}", port);

    StageServer::new(CycleDetectionProcessor)
        .port(port)
        .run()
        .await
}
