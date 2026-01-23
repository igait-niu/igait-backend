//! Stage 6: Prediction Microservice
//! 
//! Runs the ML model to predict ASD classification from gait data.

use anyhow::Result;
use async_trait::async_trait;
use igait_lib::microservice::{
    StageJobRequest, StageJobResult, StageNumber, StageProcessor, StageServer,
};
use std::collections::HashMap;
use std::time::Instant;
use tracing::{info, error};

/// The prediction processor.
pub struct PredictionProcessor;

#[async_trait]
impl StageProcessor for PredictionProcessor {
    fn stage(&self) -> StageNumber {
        StageNumber::Stage6Prediction
    }

    fn service_name(&self) -> &'static str {
        "igait-stage6-prediction"
    }

    async fn process(&self, request: StageJobRequest) -> StageJobResult {
        let start_time = Instant::now();
        let mut logs = String::new();

        info!("Processing job {}: Prediction", request.job_id);
        logs.push_str(&format!("Starting prediction for job {}\n", request.job_id));

        match self.do_prediction(&request, &mut logs).await {
            Ok(output_keys) => {
                let duration = start_time.elapsed();
                logs.push_str(&format!("Prediction completed in {:?}\n", duration));
                
                StageJobResult::success(
                    request.job_id,
                    StageNumber::Stage6Prediction,
                    output_keys,
                    logs,
                    duration.as_millis() as u64,
                )
            }
            Err(e) => {
                let duration = start_time.elapsed();
                error!("Prediction failed for job {}: {}", request.job_id, e);
                logs.push_str(&format!("ERROR: {}\n", e));
                
                StageJobResult::failure(
                    request.job_id,
                    StageNumber::Stage6Prediction,
                    e.to_string(),
                    logs,
                    duration.as_millis() as u64,
                )
            }
        }
    }
}

impl PredictionProcessor {
    async fn do_prediction(
        &self,
        request: &StageJobRequest,
        logs: &mut String,
    ) -> Result<HashMap<String, String>> {
        let cycles = request.input_keys.get("cycles")
            .ok_or_else(|| anyhow::anyhow!("Missing cycles input key"))?;
        let front_keypoints = request.input_keys.get("front_keypoints")
            .ok_or_else(|| anyhow::anyhow!("Missing front_keypoints input key"))?;

        logs.push_str(&format!("Loading cycle data: {}\n", cycles));
        logs.push_str(&format!("Loading keypoint data: {}\n", front_keypoints));

        // TODO: Run Python ML model subprocess
        // The model outputs a probability score for ASD classification

        let stage_prefix = format!("jobs/{}/stage_6", request.job_id);
        let mut output_keys = HashMap::new();
        output_keys.insert("prediction".to_string(), format!("{}/prediction.json", stage_prefix));
        
        // Include the score directly in the output for the archiver
        // In real implementation, this would come from the ML model
        output_keys.insert("score".to_string(), "0.42".to_string()); // Placeholder

        logs.push_str("Prediction complete (placeholder)\n");
        
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

    info!("Starting Stage 6 Prediction service on port {}", port);

    StageServer::new(PredictionProcessor)
        .port(port)
        .run()
        .await
}
