mod stages;

use stages::{Stages, StageData};
use std::path::PathBuf;
use clap::Parser;
use anyhow::Result;
use serde::{Serialize, Deserialize};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the front input video file
    #[arg(long, value_name = "FILE")]
    input_path_front: PathBuf,
    /// Path to the side input video file
    #[arg(long, value_name = "FILE")]
    input_path_side: PathBuf,

    /// Path to the output folder, which
    ///  is created if it does not exist
    #[arg(short, long)]
    output_dir_path: PathBuf,

    /// Whether to write the output JSON
    #[arg(short, long, default_value_t = true)]
    write_output_json: bool,

    /// Whether to submit to the webserver
    #[arg(short, long, default_value_t = false)]
    submit_to_webserver: bool,

    /// The stage to start at (1-6, optional to skip stages)
    #[arg(long)]
    skip_to_stage: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
struct CanonicalPaths {
    front_video: PathBuf,
    side_video: PathBuf,
    output_dir: PathBuf,

    stage_paths: StagePaths
}
#[derive(Serialize, Deserialize, Debug, Default)]
struct StagePaths {
    s1_media_conversion: PathBuf,
    s2_validity_check: PathBuf,
    s3_reframing: PathBuf,
    s4_pose_estimation: PathBuf,
    s5_cycle_detection: PathBuf,
    s6_prediction: PathBuf,
    s7_archive: PathBuf,
}
#[derive(Serialize, Deserialize, Debug)]
struct Output {
    canonical_paths: CanonicalPaths,
    stages: Stages,
    result: Result<f64, String>,
    skip_to_stage: Option<u8>,
}
impl Output {
    fn new(skip_to_stage: Option<u8>) -> Self {
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

async fn propagate_stage (
    stage_data: StageData,
    target: &mut Option<StageData>,
    stage_err: &str
) -> Result<(), String> {
    let has_error = stage_data.status.is_err();
    *target = Some(stage_data);
    
    if has_error {
        return Err(stage_err.into());
    }

    Ok(())
}
async fn run_pipeline(
    args: &Args,
    output: &mut Output
) -> Result<f64, String> {
    // Canonicalize paths
    output.canonical_paths = {
        let canonicalized_front_video_path = args.input_path_front.canonicalize()
            .map_err(|e| format!("Failed to canonicalize front video path: {:?}: {e:?}", args.input_path_front))?;
        let canonicalized_side_video_path = args.input_path_side.canonicalize()
            .map_err(|e| format!("Failed to canonicalize side video path: {:?}: {e:?}", args.input_path_side))?;
        let canonicalized_output_dir_path = args.output_dir_path.canonicalize()
            .map_err(|e| format!("Failed to canonicalize output dir path: {:?}: {e:?}", args.output_dir_path))?;

        let stage_paths = StagePaths {
            s1_media_conversion: canonicalized_output_dir_path.join("1_converted"),
            s2_validity_check: canonicalized_output_dir_path.join("2_validity_check"),
            s3_reframing: canonicalized_output_dir_path.join("3_reframed"),
            s4_pose_estimation: canonicalized_output_dir_path.join("4_pose_estimation"),
            s5_cycle_detection: canonicalized_output_dir_path.join("5_cycles"),
            s6_prediction: canonicalized_output_dir_path.join("6_predictions"),
            s7_archive: canonicalized_output_dir_path.join("7_archive"),
        };

        CanonicalPaths {
            front_video: canonicalized_front_video_path,
            side_video: canonicalized_side_video_path,
            output_dir: canonicalized_output_dir_path,

            stage_paths,
        }
    };

    // Build the output directory and a 
    //  subdirectory for each stage
    tokio::fs::create_dir_all(&output.canonical_paths.output_dir).await
        .map_err(|e| format!("Critical error - Failed to create output directory {:?}: {e:?}", output.canonical_paths.output_dir))?;

    if !matches!(output.skip_to_stage, Some(val) if val > 1) { propagate_stage(
        stages::s1_media_conversion::execute(
            output
        ).await,
        &mut output.stages.s1_media_conversion,
        "Critical error - Stage 1 (Media Conversion) failed"
    ).await?; }
    if !matches!(output.skip_to_stage, Some(val) if val > 2) { propagate_stage(
        stages::s2_validity_check::execute(
            output
        ).await,
        &mut output.stages.s2_validity_check,
        "Critical error - Stage 2 (Validity Check) failed"
    ).await?; }
    if !matches!(output.skip_to_stage, Some(val) if val > 3) { propagate_stage(
        stages::s3_reframing::execute(
            output
        ).await,
        &mut output.stages.s3_reframing,
        "Critical error - Stage 3 (Reframing) failed"
    ).await?; }
    if !matches!(output.skip_to_stage, Some(val) if val > 4) { propagate_stage(
        stages::s4_pose_estimation::execute(
            output
        ).await,
        &mut output.stages.s4_pose_estimation,
        "Critical error - Stage 4 (Pose Estimation) failed"
    ).await?; }
    if !matches!(output.skip_to_stage, Some(val) if val > 5) { propagate_stage(
        stages::s5_cycle_detection::execute(
            output
        ).await,
        &mut output.stages.s5_cycle_detection,
        "Critical error - Stage 5 (Cycle Detection) failed"
    ).await?; }
    if !matches!(output.skip_to_stage, Some(val) if val > 6) { propagate_stage(
        stages::s6_prediction::execute(
            output
        ).await,
        &mut output.stages.s6_prediction,
        "Critical error - Stage 6 (Prediction) failed"
    ).await?; }
    if !matches!(output.skip_to_stage, Some(val) if val > 7) { propagate_stage(
        stages::s7_archive::execute(
            output
        ).await,
        &mut output.stages.s7_archive,
        "Critical error - Stage 7 (Archive) failed"
    ).await?; }

    // Return the final prediction score (will be set by stage 7)
    output.result.clone()
}
async fn main_wrapper (
    args: &Args,
    output: &mut Output
) {
    output.result = run_pipeline(args, output).await;

    if args.write_output_json {
        tokio::fs::write(
            output.canonical_paths.output_dir.join("output.json"),
            serde_json::to_string_pretty(&output).unwrap()
        ).await
            .expect("Critical error - Failed to write output.json");
    }
}
#[tokio::main]
async fn main() {
    let args = Args::parse();
    let mut output = Output::new(args.skip_to_stage);

    main_wrapper(&args, &mut output).await;

    println!("{output:#?}");
}
