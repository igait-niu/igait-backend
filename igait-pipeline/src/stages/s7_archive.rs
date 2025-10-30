use std::fs::File;
use std::io::{Read, Write};
use walkdir::WalkDir;
use zip::write::FileOptions;
use zip::ZipWriter;

use igait_lib::{StageData, StageStatus, Output};
use anyhow::{anyhow, ensure, Context, Result};

async fn create_archive (
    output: &mut Output,
    logs: &mut String
) -> Result<StageStatus> {
    // Check if we're skipping to this stage or later
    let is_skipping = output.skip_to_stage.map_or(false, |skip| skip >= 7);
    
    let prediction_output_dir = &output.canonical_paths.stage_paths.s6_prediction;
    let results_csv_path = prediction_output_dir.join("Results/predictions.csv");
    
    if is_skipping {
        // When skipping, verify required files exist from stage 6
        if !tokio::fs::try_exists(&results_csv_path).await
            .context("Failed to check if predictions.csv exists")? {
            anyhow::bail!("Skipping to Stage 7 but required files from Stage 6 (Prediction) don't exist at: {}", results_csv_path.display());
        }
        logs.push_str("Skipping to Stage 7: Verified required files from Stage 6 exist\n");
    } else {
        // When not skipping, verify previous stage completed successfully
        let prediction_stage = output.stages.s6_prediction
            .as_ref()
            .context("Stage 6 (Prediction) data missing")?
            .status
            .as_ref()
            .map_err(|e| anyhow!("Stage 6 (Prediction) failed: {e}"))?;
        ensure!(matches!(prediction_stage, StageStatus::Done), "Stage 6 (Prediction) did not complete successfully");
    }
    
    // Read the prediction results from CSV
    logs.push_str("Reading prediction results from CSV...\n");
    let mut rdr = csv::Reader::from_path(&results_csv_path)
        .context("Failed to open predictions.csv")?;
    
    let mut prediction_score = 0.0;
    
    for result in rdr.records() {
        let record = result.context("Failed to read CSV record")?;
        if record.len() >= 2 {
            prediction_score = record[0].parse::<f64>()
                .context("Failed to parse prediction score")?;
            let prediction_label = &record[1];
            logs.push_str(&format!("Prediction: {} (Score: {:.4})\n", prediction_label, prediction_score));
            break;
        }
    }
    
    // Store the result in the output
    output.result = Ok(prediction_score);
    
    // Create stage 7 output directory
    let archive_output_dir = &output.canonical_paths.stage_paths.s7_archive;
    tokio::fs::create_dir_all(archive_output_dir).await
        .context("Failed to create archive output directory")?;
    
    // Create zip file with all results
    let zip_path = archive_output_dir.join("results.zip");
    logs.push_str(&format!("Creating archive at: {}\n", zip_path.display()));
    
    let file = File::create(&zip_path)
        .context("Failed to create zip file")?;
    let mut zip = ZipWriter::new(file);
    let options: FileOptions<()> = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);
    
    // Add all files from stage 6 prediction output to the zip
    logs.push_str("Adding files to archive...\n");
    let base_path = prediction_output_dir;
    
    for entry in WalkDir::new(base_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        let name = path.strip_prefix(base_path)
            .context("Failed to strip prefix from path")?;
        
        // Skip the root directory
        if name.as_os_str().is_empty() {
            continue;
        }
        
        if path.is_file() {
            logs.push_str(&format!("  Adding: {}\n", name.display()));
            zip.start_file(name.to_string_lossy().to_string(), options)
                .context("Failed to start file in zip")?;
            
            let mut f = File::open(path)
                .context("Failed to open file for zipping")?;
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer)
                .context("Failed to read file contents")?;
            zip.write_all(&buffer)
                .context("Failed to write file to zip")?;
        } else if path.is_dir() {
            // Add directory entry
            let dir_name = format!("{}/", name.to_string_lossy());
            if dir_name != "/" {
                zip.add_directory(dir_name, options)
                    .context("Failed to add directory to zip")?;
            }
        }
    }
    
    zip.finish().context("Failed to finalize zip file")?;
    logs.push_str(&format!("Archive created successfully: {}\n", zip_path.display()));
    logs.push_str(&format!("Final prediction score: {:.4}\n", prediction_score));
    
    Ok(StageStatus::Done)
}

pub async fn execute(
    output: &mut Output
) -> StageData {
    let mut logs = String::new();

    let status = match create_archive(output, &mut logs).await {
        Ok(status) => Ok(status),
        Err(e) => {
            let mut error_body = format!("{e:?}\n");

            for (ind, err) in e.chain().enumerate() {
                error_body.push_str(&format!("{ind}: {err}\n"));
            }

            Err(error_body)
        }
    };

    StageData {
        status,
        logs,
    }
}
