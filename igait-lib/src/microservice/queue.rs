//! Queue types and operations for the distributed job processing pipeline.
//!
//! This module defines the data structures used for Firebase Realtime Database
//! queue-based job processing with claim-based distributed locking.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::microservice::{JobMetadata, StageNumber};

/// Default timeout for claimed jobs (50 minutes in milliseconds).
/// If a worker claims a job but doesn't complete it within this time,
/// the job becomes available for other workers to claim.
pub const CLAIM_TIMEOUT_MS: u64 = 50 * 60 * 1000;

/// Interval for heartbeat updates during long-running jobs (1 minute).
pub const HEARTBEAT_INTERVAL_SECS: u64 = 60;

// ============================================================================
// QUEUE ITEM TYPES
// ============================================================================

/// An item in a stage processing queue.
/// 
/// This represents a job waiting to be processed by a specific stage.
/// Workers claim items using Firebase transactions to prevent duplicate processing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueItem {
    /// The job ID (format: "{user_id}_{job_index}")
    pub job_id: String,
    
    /// User ID who owns this job
    pub user_id: String,
    
    /// When the item was added to this queue (Unix timestamp ms)
    pub enqueued_at: u64,
    
    /// Worker ID that claimed this job (None if unclaimed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claimed_by: Option<String>,
    
    /// When the job was claimed (Unix timestamp ms, for timeout detection)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claimed_at: Option<u64>,
    
    /// Storage keys for input files from previous stage
    pub input_keys: HashMap<String, String>,
    
    /// Job metadata (age, sex, etc. - needed for later stages)
    pub metadata: JobMetadata,
}

impl QueueItem {
    /// Creates a new unclaimed queue item.
    pub fn new(
        job_id: String,
        user_id: String,
        input_keys: HashMap<String, String>,
        metadata: JobMetadata,
    ) -> Self {
        Self {
            job_id,
            user_id,
            enqueued_at: now_ms(),
            claimed_by: None,
            claimed_at: None,
            input_keys,
            metadata,
        }
    }

    /// Checks if this item is available for claiming.
    /// 
    /// An item is available if:
    /// - It has never been claimed, OR
    /// - It was claimed but the claim has timed out
    pub fn is_available(&self) -> bool {
        match self.claimed_at {
            None => true, // Never claimed
            Some(claimed_time) => {
                // Check if claim has timed out
                now_ms().saturating_sub(claimed_time) > CLAIM_TIMEOUT_MS
            }
        }
    }

    /// Claims this item for a worker.
    /// 
    /// Returns the modified item with claim information set.
    pub fn claim(&self, worker_id: &str) -> Self {
        Self {
            claimed_by: Some(worker_id.to_string()),
            claimed_at: Some(now_ms()),
            ..self.clone()
        }
    }

    /// Updates the heartbeat timestamp to prevent timeout during long operations.
    pub fn heartbeat(&self) -> Self {
        Self {
            claimed_at: Some(now_ms()),
            ..self.clone()
        }
    }

    /// Gets the input storage key for the front video from the input_keys.
    /// Falls back to constructing from job_id if not present.
    pub fn input_front_video(&self, stage: StageNumber) -> String {
        self.input_keys
            .get("front_video")
            .cloned()
            .unwrap_or_else(|| {
                let prev_stage = stage.as_u8().saturating_sub(1);
                format!("jobs/{}/stage_{}/front.mp4", self.job_id, prev_stage)
            })
    }

    /// Gets the input storage key for the side video from the input_keys.
    /// Falls back to constructing from job_id if not present.
    pub fn input_side_video(&self, stage: StageNumber) -> String {
        self.input_keys
            .get("side_video")
            .cloned()
            .unwrap_or_else(|| {
                let prev_stage = stage.as_u8().saturating_sub(1);
                format!("jobs/{}/stage_{}/side.mp4", self.job_id, prev_stage)
            })
    }

    /// Gets the output storage key for the front video for a given stage.
    pub fn output_front_video(&self, stage: StageNumber) -> String {
        format!("jobs/{}/stage_{}/front.mp4", self.job_id, stage.as_u8())
    }

    /// Gets the output storage key for the side video for a given stage.
    pub fn output_side_video(&self, stage: StageNumber) -> String {
        format!("jobs/{}/stage_{}/side.mp4", self.job_id, stage.as_u8())
    }
}

/// An item in the finalize queue.
/// 
/// This queue receives jobs that have either:
/// - Successfully completed all stages (success = true)
/// - Failed at some stage (success = false)
/// 
/// The finalize worker sends appropriate emails and updates the database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinalizeQueueItem {
    /// The job ID (format: "{user_id}_{job_index}")
    pub job_id: String,
    
    /// User ID who owns this job
    pub user_id: String,
    
    /// When the item was added to this queue (Unix timestamp ms)
    pub enqueued_at: u64,
    
    /// Worker ID that claimed this job (None if unclaimed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claimed_by: Option<String>,
    
    /// When the job was claimed (Unix timestamp ms, for timeout detection)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claimed_at: Option<u64>,
    
    /// Whether the pipeline completed successfully
    pub success: bool,
    
    /// If failed, which stage failed (1-6)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_at_stage: Option<u8>,
    
    /// Error message if failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    
    /// Error logs if failed (for debugging)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_logs: Option<String>,
    
    /// Final output keys (if successful - includes prediction results)
    pub output_keys: HashMap<String, String>,
    
    /// Job metadata for email content
    pub metadata: JobMetadata,
}

impl FinalizeQueueItem {
    /// Creates a success finalize item (pipeline completed successfully).
    pub fn success(
        job_id: String,
        user_id: String,
        output_keys: HashMap<String, String>,
        metadata: JobMetadata,
    ) -> Self {
        Self {
            job_id,
            user_id,
            enqueued_at: now_ms(),
            claimed_by: None,
            claimed_at: None,
            success: true,
            failed_at_stage: None,
            error: None,
            error_logs: None,
            output_keys,
            metadata,
        }
    }

    /// Creates a failure finalize item (stage failed).
    pub fn failure(
        job_id: String,
        user_id: String,
        failed_at_stage: u8,
        error: String,
        error_logs: Option<String>,
        metadata: JobMetadata,
    ) -> Self {
        Self {
            job_id,
            user_id,
            enqueued_at: now_ms(),
            claimed_by: None,
            claimed_at: None,
            success: false,
            failed_at_stage: Some(failed_at_stage),
            error: Some(error),
            error_logs,
            output_keys: HashMap::new(),
            metadata,
        }
    }

    /// Checks if this item is available for claiming.
    pub fn is_available(&self) -> bool {
        match self.claimed_at {
            None => true,
            Some(claimed_time) => {
                now_ms().saturating_sub(claimed_time) > CLAIM_TIMEOUT_MS
            }
        }
    }

    /// Claims this item for a worker.
    pub fn claim(&self, worker_id: &str) -> Self {
        Self {
            claimed_by: Some(worker_id.to_string()),
            claimed_at: Some(now_ms()),
            ..self.clone()
        }
    }
}

// ============================================================================
// QUEUE PATH HELPERS
// ============================================================================

/// Returns the Firebase RTDB path for a stage's queue.
/// 
/// Queue paths are: `queues/stage_{n}` for stages 1-6, `queues/finalize` for stage 7.
pub fn queue_path(stage: StageNumber) -> String {
    match stage {
        StageNumber::Stage7Finalize => "queues/finalize".to_string(),
        other => format!("queues/stage_{}", other.as_u8()),
    }
}

/// Returns the Firebase RTDB path for a specific job in a queue.
pub fn queue_item_path(stage: StageNumber, job_id: &str) -> String {
    // Replace characters that Firebase doesn't allow in keys
    let safe_job_id = job_id.replace('.', "_").replace('/', "_");
    format!("{}/{}", queue_path(stage), safe_job_id)
}

/// Returns the next stage in the pipeline, or Stage7Finalize if at the end.
pub fn next_stage(current: StageNumber) -> StageNumber {
    match current {
        StageNumber::Stage1MediaConversion => StageNumber::Stage2ValidityCheck,
        StageNumber::Stage2ValidityCheck => StageNumber::Stage3Reframing,
        StageNumber::Stage3Reframing => StageNumber::Stage4PoseEstimation,
        StageNumber::Stage4PoseEstimation => StageNumber::Stage5CycleDetection,
        StageNumber::Stage5CycleDetection => StageNumber::Stage6Prediction,
        StageNumber::Stage6Prediction => StageNumber::Stage7Finalize,
        StageNumber::Stage7Finalize => StageNumber::Stage7Finalize, // Terminal
    }
}

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

/// Returns the current Unix timestamp in milliseconds.
pub fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

/// Generates a unique worker ID for this instance.
/// 
/// Format: `{service_name}_{hostname}_{pid}_{random}`
pub fn generate_worker_id(service_name: &str) -> String {
    let hostname = std::env::var("HOSTNAME")
        .or_else(|_| std::env::var("POD_NAME"))
        .unwrap_or_else(|_| "unknown".to_string());
    let pid = std::process::id();
    let random: u32 = rand_u32();
    
    format!("{}_{}_{}_{:08x}", service_name, hostname, pid, random)
}

/// Simple pseudo-random u32 (not cryptographically secure, just for IDs).
fn rand_u32() -> u32 {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};
    
    let state = RandomState::new();
    let mut hasher = state.build_hasher();
    hasher.write_u64(now_ms());
    hasher.write_u32(std::process::id());
    hasher.finish() as u32
}

// ============================================================================
// QUEUE OPERATION RESULTS
// ============================================================================

/// Result of attempting to claim a job from a queue.
#[derive(Debug, Clone)]
pub enum ClaimResult<T> {
    /// Successfully claimed a job
    Claimed(T),
    /// No jobs available in the queue
    QueueEmpty,
    /// Jobs exist but all are claimed by other workers
    AllClaimed,
    /// Error occurred during claim operation
    Error(String),
}

/// Result of a stage processing operation.
#[derive(Debug, Clone)]
pub enum ProcessingResult {
    /// Stage completed successfully with output keys
    Success {
        output_keys: HashMap<String, String>,
        logs: String,
        duration_ms: u64,
    },
    /// Stage failed with an error
    Failure {
        error: String,
        logs: String,
        duration_ms: u64,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue_paths() {
        assert_eq!(queue_path(StageNumber::Stage1MediaConversion), "queues/stage_1");
        assert_eq!(queue_path(StageNumber::Stage6Prediction), "queues/stage_6");
        assert_eq!(queue_path(StageNumber::Stage7Finalize), "queues/finalize");
    }

    #[test]
    fn test_next_stage() {
        assert_eq!(next_stage(StageNumber::Stage1MediaConversion), StageNumber::Stage2ValidityCheck);
        assert_eq!(next_stage(StageNumber::Stage6Prediction), StageNumber::Stage7Finalize);
        assert_eq!(next_stage(StageNumber::Stage7Finalize), StageNumber::Stage7Finalize);
    }

    #[test]
    fn test_queue_item_availability() {
        let item = QueueItem::new(
            "test_job".to_string(),
            "test_user".to_string(),
            HashMap::new(),
            JobMetadata::default(),
        );
        
        assert!(item.is_available());
        
        let claimed = item.claim("worker_1");
        assert!(!claimed.is_available()); // Just claimed, not timed out yet
    }
}
