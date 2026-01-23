//! Stage 2: Validity Check Microservice
//! 
//! Verifies that a person can be detected in the uploaded videos.
//! Uses OpenCV/MediaPipe for person detection.

use anyhow::Result;
use async_trait::async_trait;
use igait_lib::microservice::{
    StageJobRequest, StageJobResult, StageNumber, StageProcessor, StageServer,
};
use std::collections::HashMap;
use std::time::Instant;
use tracing::{info, error};

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

        info!("Processing job {}: Validity Check", request.job_id);
        logs.push_str(&format!("Starting validity check for job {}\n", request.job_id));

        match self.do_validity_check(&request, &mut logs).await {
            Ok(output_keys) => {
                let duration = start_time.elapsed();
                logs.push_str(&format!("Validity check completed in {:?}\n", duration));
                
                StageJobResult::success(
                    request.job_id,
                    StageNumber::Stage2ValidityCheck,
                    output_keys,
                    logs,
                    duration.as_millis() as u64,
                )
            }
            Err(e) => {
                let duration = start_time.elapsed();
                error!("Validity check failed for job {}: {}", request.job_id, e);
                logs.push_str(&format!("ERROR: {}\n", e));
                
                StageJobResult::failure(
                    request.job_id,
                    StageNumber::Stage2ValidityCheck,
                    e.to_string(),
                    logs,
                    duration.as_millis() as u64,
                )
            }
        }
    }
}

impl ValidityCheckProcessor {
    async fn do_validity_check(
        &self,
        request: &StageJobRequest,
        logs: &mut String,
    ) -> Result<HashMap<String, String>> {
        // Get input file paths from request
        let front_input = request.input_keys.get("front_video")
            .ok_or_else(|| anyhow::anyhow!("Missing front_video input key"))?;
        let side_input = request.input_keys.get("side_video")
            .ok_or_else(|| anyhow::anyhow!("Missing side_video input key"))?;

        logs.push_str(&format!("Checking front video: {}\n", front_input));
        logs.push_str(&format!("Checking side video: {}\n", side_input));

        // TODO: Download files from storage
        // TODO: Run person detection (Python subprocess or native)
        // TODO: Upload validation report

        // For now, return placeholder output keys
        let stage_prefix = format!("jobs/{}/stage_2", request.job_id);
        let mut output_keys = HashMap::new();
        output_keys.insert("validation_report".to_string(), format!("{}/validation.json", stage_prefix));
        // Pass through video keys for next stage
        output_keys.insert("front_video".to_string(), front_input.clone());
        output_keys.insert("side_video".to_string(), side_input.clone());

        logs.push_str("Validity check complete (placeholder)\n");
        
        Ok(output_keys)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);

    info!("Starting Stage 2 Validity Check service on port {}", port);

    StageServer::new(ValidityCheckProcessor)
        .port(port)
        .run()
        .await
}
