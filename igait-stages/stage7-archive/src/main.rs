//! Stage 7: Archive Microservice
//! 
//! Packages all pipeline results into a final ZIP archive.

use anyhow::Result;
use async_trait::async_trait;
use igait_lib::microservice::{
    StageJobRequest, StageJobResult, StageNumber, StageProcessor, StageServer,
};
use std::collections::HashMap;
use std::time::Instant;
use tracing::{info, error};

/// The archive processor.
pub struct ArchiveProcessor;

#[async_trait]
impl StageProcessor for ArchiveProcessor {
    fn stage(&self) -> StageNumber {
        StageNumber::Stage7Archive
    }

    fn service_name(&self) -> &'static str {
        "igait-stage7-archive"
    }

    async fn process(&self, request: StageJobRequest) -> StageJobResult {
        let start_time = Instant::now();
        let mut logs = String::new();

        info!("Processing job {}: Archive", request.job_id);
        logs.push_str(&format!("Starting archival for job {}\n", request.job_id));

        match self.do_archive(&request, &mut logs).await {
            Ok(output_keys) => {
                let duration = start_time.elapsed();
                logs.push_str(&format!("Archival completed in {:?}\n", duration));
                
                StageJobResult::success(
                    request.job_id,
                    StageNumber::Stage7Archive,
                    output_keys,
                    logs,
                    duration.as_millis() as u64,
                )
            }
            Err(e) => {
                let duration = start_time.elapsed();
                error!("Archival failed for job {}: {}", request.job_id, e);
                logs.push_str(&format!("ERROR: {}\n", e));
                
                StageJobResult::failure(
                    request.job_id,
                    StageNumber::Stage7Archive,
                    e.to_string(),
                    logs,
                    duration.as_millis() as u64,
                )
            }
        }
    }
}

impl ArchiveProcessor {
    async fn do_archive(
        &self,
        request: &StageJobRequest,
        logs: &mut String,
    ) -> Result<HashMap<String, String>> {
        let prediction = request.input_keys.get("prediction")
            .ok_or_else(|| anyhow::anyhow!("Missing prediction input key"))?;

        logs.push_str(&format!("Including prediction results: {}\n", prediction));

        // TODO: Download all stage outputs
        // TODO: Create ZIP archive
        // TODO: Upload to storage

        let stage_prefix = format!("jobs/{}/stage_7", request.job_id);
        let mut output_keys = HashMap::new();
        output_keys.insert("archive".to_string(), format!("{}/results.zip", stage_prefix));
        
        // Pass through the score for the backend
        if let Some(score) = request.input_keys.get("score") {
            output_keys.insert("score".to_string(), score.clone());
        }

        logs.push_str("Archival complete (placeholder)\n");
        
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

    info!("Starting Stage 7 Archive service on port {}", port);

    StageServer::new(ArchiveProcessor)
        .port(port)
        .run()
        .await
}
