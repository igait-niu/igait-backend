//! Stage 3: Reframing Microservice
//! 
//! Adjusts video framing and cropping based on detected person position.

use anyhow::Result;
use async_trait::async_trait;
use igait_lib::microservice::{
    StageJobRequest, StageJobResult, StageNumber, StageProcessor, StageServer,
};
use std::collections::HashMap;
use std::time::Instant;
use tracing::{info, error};

/// The reframing processor.
pub struct ReframingProcessor;

#[async_trait]
impl StageProcessor for ReframingProcessor {
    fn stage(&self) -> StageNumber {
        StageNumber::Stage3Reframing
    }

    fn service_name(&self) -> &'static str {
        "igait-stage3-reframing"
    }

    async fn process(&self, request: StageJobRequest) -> StageJobResult {
        let start_time = Instant::now();
        let mut logs = String::new();

        info!("Processing job {}: Reframing", request.job_id);
        logs.push_str(&format!("Starting reframing for job {}\n", request.job_id));

        match self.do_reframing(&request, &mut logs).await {
            Ok(output_keys) => {
                let duration = start_time.elapsed();
                logs.push_str(&format!("Reframing completed in {:?}\n", duration));
                
                StageJobResult::success(
                    request.job_id,
                    StageNumber::Stage3Reframing,
                    output_keys,
                    logs,
                    duration.as_millis() as u64,
                )
            }
            Err(e) => {
                let duration = start_time.elapsed();
                error!("Reframing failed for job {}: {}", request.job_id, e);
                logs.push_str(&format!("ERROR: {}\n", e));
                
                StageJobResult::failure(
                    request.job_id,
                    StageNumber::Stage3Reframing,
                    e.to_string(),
                    logs,
                    duration.as_millis() as u64,
                )
            }
        }
    }
}

impl ReframingProcessor {
    async fn do_reframing(
        &self,
        request: &StageJobRequest,
        logs: &mut String,
    ) -> Result<HashMap<String, String>> {
        let front_input = request.input_keys.get("front_video")
            .ok_or_else(|| anyhow::anyhow!("Missing front_video input key"))?;
        let side_input = request.input_keys.get("side_video")
            .ok_or_else(|| anyhow::anyhow!("Missing side_video input key"))?;

        logs.push_str(&format!("Reframing front video: {}\n", front_input));
        logs.push_str(&format!("Reframing side video: {}\n", side_input));

        // TODO: Implement reframing logic

        let stage_prefix = format!("jobs/{}/stage_3", request.job_id);
        let mut output_keys = HashMap::new();
        output_keys.insert("front_video".to_string(), format!("{}/front.mp4", stage_prefix));
        output_keys.insert("side_video".to_string(), format!("{}/side.mp4", stage_prefix));

        logs.push_str("Reframing complete (placeholder)\n");
        
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

    info!("Starting Stage 3 Reframing service on port {}", port);

    StageServer::new(ReframingProcessor)
        .port(port)
        .run()
        .await
}
