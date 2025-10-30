use serde::{Serialize, Deserialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CanonicalPaths {
    pub front_video: PathBuf,
    pub side_video: PathBuf,
    pub output_dir: PathBuf,
    pub stage_paths: StagePaths
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct StagePaths {
    pub s1_media_conversion: PathBuf,
    pub s2_validity_check: PathBuf,
    pub s3_reframing: PathBuf,
    pub s4_pose_estimation: PathBuf,
    pub s5_cycle_detection: PathBuf,
    pub s6_prediction: PathBuf,
    pub s7_archive: PathBuf,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Output {
    pub canonical_paths: CanonicalPaths,
    pub stages: Stages,
    pub result: Result<f64, String>,
    pub skip_to_stage: Option<u8>,
}
impl Output {
    pub fn new(skip_to_stage: Option<u8>) -> Self {
        Self {
            canonical_paths: CanonicalPaths {
                front_video: PathBuf::default(),
                side_video: PathBuf::default(),
                output_dir: PathBuf::default(),
                stage_paths: StagePaths::default()
            },
            stages: Stages::default(),
            result: Err("Critical error - Pipeline failed before starting".to_string()),
            skip_to_stage,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Stages {
    pub s1_media_conversion: Option<StageData>,
    pub s2_validity_check: Option<StageData>,
    pub s3_reframing: Option<StageData>,
    pub s4_pose_estimation: Option<StageData>,
    pub s5_cycle_detection: Option<StageData>,
    pub s6_prediction: Option<StageData>,
    pub s7_archive: Option<StageData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StageData {
    pub status: Result<StageStatus, String>,
    pub logs: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum StageStatus {
    Done,
    Skipped
}
