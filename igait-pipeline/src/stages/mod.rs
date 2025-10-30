pub mod s1_media_conversion;
pub mod s2_validity_check;
pub mod s3_reframing;
pub mod s4_pose_estimation;
pub mod s5_cycle_detection;
pub mod s6_prediction;
pub mod s7_archive;

use anyhow::Result;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum StageStatus {
    Skipped,
    Done
}
#[derive(Serialize, Deserialize, Debug)]
pub struct StageData {
    pub status: Result<StageStatus, String>,
    pub logs: String,
}
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Stages {
    pub s1_media_conversion: Option<StageData>,
    pub s2_validity_check:   Option<StageData>,
    pub s3_reframing:        Option<StageData>,
    pub s4_pose_estimation:  Option<StageData>,
    pub s5_cycle_detection:  Option<StageData>,
    pub s6_prediction:       Option<StageData>,
    pub s7_archive:          Option<StageData>,
}