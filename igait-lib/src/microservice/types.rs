//! Core types for microservice communication.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// Identifies which stage a microservice handles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StageNumber {
    Stage1MediaConversion,
    Stage2ValidityCheck,
    Stage3Reframing,
    Stage4PoseEstimation,
    Stage5CycleDetection,
    Stage6Prediction,
    Stage7Archive,
}

impl StageNumber {
    /// Returns the numeric stage number (1-7).
    pub fn as_u8(&self) -> u8 {
        match self {
            Self::Stage1MediaConversion => 1,
            Self::Stage2ValidityCheck => 2,
            Self::Stage3Reframing => 3,
            Self::Stage4PoseEstimation => 4,
            Self::Stage5CycleDetection => 5,
            Self::Stage6Prediction => 6,
            Self::Stage7Archive => 7,
        }
    }

    /// Creates a StageNumber from a u8 (1-7).
    pub fn from_u8(n: u8) -> Option<Self> {
        match n {
            1 => Some(Self::Stage1MediaConversion),
            2 => Some(Self::Stage2ValidityCheck),
            3 => Some(Self::Stage3Reframing),
            4 => Some(Self::Stage4PoseEstimation),
            5 => Some(Self::Stage5CycleDetection),
            6 => Some(Self::Stage6Prediction),
            7 => Some(Self::Stage7Archive),
            _ => None,
        }
    }

    /// Returns the human-readable name for this stage.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Stage1MediaConversion => "Media Conversion",
            Self::Stage2ValidityCheck => "Validity Check",
            Self::Stage3Reframing => "Reframing",
            Self::Stage4PoseEstimation => "Pose Estimation",
            Self::Stage5CycleDetection => "Cycle Detection",
            Self::Stage6Prediction => "Prediction",
            Self::Stage7Archive => "Archive",
        }
    }

    /// Returns the storage path prefix for this stage's outputs.
    pub fn storage_prefix(&self) -> &'static str {
        match self {
            Self::Stage1MediaConversion => "stage_1",
            Self::Stage2ValidityCheck => "stage_2",
            Self::Stage3Reframing => "stage_3",
            Self::Stage4PoseEstimation => "stage_4",
            Self::Stage5CycleDetection => "stage_5",
            Self::Stage6Prediction => "stage_6",
            Self::Stage7Archive => "stage_7",
        }
    }
}

// ============================================================================
// JOB REQUEST/RESPONSE TYPES
// ============================================================================

/// Request sent to a stage microservice to process a job.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageJobRequest {
    /// Unique job identifier (format: "{user_id}_{job_index}")
    pub job_id: String,
    
    /// User ID who owns this job
    pub user_id: String,
    
    /// Which stage this request is for
    pub stage: StageNumber,
    
    /// URL to call when processing completes (backend webhook)
    pub callback_url: String,
    
    /// Storage keys for input files (from previous stage)
    /// Keys are descriptive names, values are storage paths
    pub input_keys: HashMap<String, String>,
    
    /// Optional metadata for stage-specific configuration
    #[serde(default)]
    pub metadata: JobMetadata,
}

impl StageJobRequest {
    /// Gets the input storage key for the front video from the previous stage.
    /// For stage 1, this is the uploaded file (stage_0).
    pub fn input_front_video(&self) -> String {
        let prev_stage = self.stage.as_u8().saturating_sub(1);
        format!("jobs/{}/stage_{}/front.mp4", self.job_id, prev_stage)
    }

    /// Gets the input storage key for the side video from the previous stage.
    /// For stage 1, this is the uploaded file (stage_0).
    pub fn input_side_video(&self) -> String {
        let prev_stage = self.stage.as_u8().saturating_sub(1);
        format!("jobs/{}/stage_{}/side.mp4", self.job_id, prev_stage)
    }

    /// Gets the output storage key for the front video for this stage.
    pub fn output_front_video(&self) -> String {
        format!("jobs/{}/stage_{}/front.mp4", self.job_id, self.stage.as_u8())
    }

    /// Gets the output storage key for the side video for this stage.
    pub fn output_side_video(&self) -> String {
        format!("jobs/{}/stage_{}/side.mp4", self.job_id, self.stage.as_u8())
    }
}

/// Optional metadata that can be passed to stages.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JobMetadata {
    /// Patient age (for stages that need it)
    pub age: Option<i16>,
    
    /// Patient sex (for stages that need it)
    pub sex: Option<char>,
    
    /// Any additional key-value pairs
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Result sent back to the backend after processing completes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageJobResult {
    /// The job ID that was processed
    pub job_id: String,
    
    /// Which stage completed
    pub stage: StageNumber,
    
    /// Whether processing succeeded or failed
    pub status: StageResultStatus,
    
    /// Storage keys for output files (for next stage)
    pub output_keys: HashMap<String, String>,
    
    /// Processing logs (for debugging)
    pub logs: String,
    
    /// How long processing took in milliseconds
    pub duration_ms: u64,
    
    /// Error message if status is Failed
    pub error: Option<String>,
}

impl StageJobResult {
    /// Creates a successful result.
    pub fn success(
        job_id: String,
        stage: StageNumber,
        output_keys: HashMap<String, String>,
        logs: String,
        duration_ms: u64,
    ) -> Self {
        Self {
            job_id,
            stage,
            status: StageResultStatus::Success,
            output_keys,
            logs,
            duration_ms,
            error: None,
        }
    }

    /// Creates a failed result.
    pub fn failure(
        job_id: String,
        stage: StageNumber,
        error: String,
        logs: String,
        duration_ms: u64,
    ) -> Self {
        Self {
            job_id,
            stage,
            status: StageResultStatus::Failed,
            output_keys: HashMap::new(),
            logs,
            duration_ms,
            error: Some(error),
        }
    }
}

/// Status of a completed stage job.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StageResultStatus {
    Success,
    Failed,
    Skipped,
}

// ============================================================================
// HEALTH & STATUS TYPES
// ============================================================================

/// Health check response from a stage service.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    /// Whether the service is healthy
    pub healthy: bool,
    
    /// Service name
    pub service: String,
    
    /// Which stage this service handles
    pub stage: StageNumber,
    
    /// Service version
    pub version: String,
    
    /// Current timestamp
    pub timestamp: DateTime<Utc>,
    
    /// Number of jobs currently processing
    pub jobs_processing: usize,
    
    /// Number of jobs in queue
    pub jobs_queued: usize,
}

/// Status of a job being processed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobStatusResponse {
    /// The job ID
    pub job_id: String,
    
    /// Current status
    pub status: JobProgress,
    
    /// When the job was received
    pub received_at: DateTime<Utc>,
    
    /// When processing started (if started)
    pub started_at: Option<DateTime<Utc>>,
    
    /// Progress percentage (0-100, if available)
    pub progress_percent: Option<u8>,
}

/// Progress state of a job.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JobProgress {
    Queued,
    Processing,
    Completed,
    Failed,
    NotFound,
}

// ============================================================================
// FIRESTORE JOB DOCUMENT (for backend/frontend)
// ============================================================================

/// Firestore document representing a complete job.
/// This is the source of truth for job state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirestoreJob {
    /// Unique job identifier
    pub job_id: String,
    
    /// User ID who owns this job
    pub user_id: String,
    
    /// When the job was created
    pub created_at: DateTime<Utc>,
    
    /// When the job was last updated
    pub updated_at: DateTime<Utc>,
    
    /// Patient information
    pub patient: PatientInfo,
    
    /// Overall job status
    pub status: FirestoreJobStatus,
    
    /// Current stage number (0 = uploaded, 1-7 = processing that stage)
    pub current_stage: u8,
    
    /// Per-stage results
    pub stages: HashMap<String, FirestoreStageResult>,
    
    /// Final result (populated after stage 7)
    pub result: Option<FinalResult>,
    
    /// Email for notifications
    pub email: String,
    
    /// Whether completion email has been sent
    pub email_sent: bool,
}

/// Patient demographic information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientInfo {
    pub age: i16,
    pub sex: char,
    pub height: String,
    pub weight: i16,
    pub ethnicity: String,
}

/// Overall job status in Firestore.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FirestoreJobStatus {
    Submitted,
    Processing,
    Completed,
    Failed,
}

/// Per-stage result stored in Firestore.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirestoreStageResult {
    pub status: FirestoreStageStatus,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub duration_ms: Option<u64>,
    pub output_keys: Option<Vec<String>>,
    pub error: Option<String>,
}

/// Stage status in Firestore.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FirestoreStageStatus {
    Pending,
    Processing,
    Success,
    Failed,
    Skipped,
}

/// Final prediction result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinalResult {
    /// ASD probability score (0.0 - 1.0)
    pub score: f64,
    
    /// Classification result
    pub classification: String,
    
    /// Storage key for the results archive
    pub archive_key: String,
}
