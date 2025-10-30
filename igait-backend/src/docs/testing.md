# Local Testing Guide

This guide explains how to test the pipeline→backend integration locally with just two terminal windows.

## Prerequisites

1. Ensure `.env` file exists in the workspace root with:
   ```bash
   PIPELINE_SECRET=MEOWMEOWMEOW
   BACKEND_URL=http://localhost:3000
   ```

2. Ensure test video files exist:
   - `~/Su_101_F_07_0_50.5.MOV` (front video)
   - `~/Su_101_S_07_0_50.5.MOV` (side video)

## Testing Steps

### Terminal 1: Run the Backend
```bash
cd /lstr/sahara/zwlab/jw/igait-pipeline
cargo run -p igait-backend
```

Wait for the backend to start. You should see:
```
[3/3] Starting iGait backend on port 3000...
```

### Terminal 2: Run the Pipeline with Submission
```bash
cd /lstr/sahara/zwlab/jw/igait-pipeline
bash test_pipeline_submit.sh
```

Or run directly:
```bash
cargo build --release -p igait-pipeline
./target/release/igait-pipeline \
    --input-path-front ~/Su_101_F_07_0_50.5.MOV \
    --input-path-side ~/Su_101_S_07_0_50.5.MOV \
    --output-dir-path ./output \
    --skip-to-stage 6 \
    --submit-to-webserver
```

## What to Look For

### In Terminal 2 (Pipeline):
- Pipeline stages executing (skipping to stage 6)
- Stage 6 (Prediction) completing
- Stage 7 (Archive) creating results.zip
- Message: `Successfully submitted results to webserver`

### In Terminal 1 (Backend):
You should see logs like:
```
INFO Processing pipeline submission for job ID: output
INFO Job output completed with score 0.5140 (ASD)
INFO Uploading results archive for job ID output to S3
INFO Successfully uploaded archive to S3: results/output/results.zip
WARN Job ID format unexpected (should be uid_jobindex): output
```

**Note:** The warning about job ID format is expected in local testing since we're using "output" as the job ID instead of "uid_jobindex".

## Testing Without Submission

To test the pipeline without webserver submission (original behavior):
```bash
bash test.sh
```

This will:
- Skip to stage 6 (for speed)
- Write output.json to ./output/
- NOT submit to webserver

## Simulating Production Job ID Format

To test with a proper job ID format like production uses:
```bash
./target/release/igait-pipeline \
    --input-path-front ~/Su_101_F_07_0_50.5.MOV \
    --input-path-side ~/Su_101_S_07_0_50.5.MOV \
    --output-dir-path ./output_testuser_5 \
    --skip-to-stage 6 \
    --submit-to-webserver
```

This uses `testuser_5` as the job ID (user "testuser", job index 5).

## Troubleshooting

### "Connection refused" error
- Make sure the backend is running in Terminal 1
- Check that BACKEND_URL in .env is `http://localhost:3000`

### "Invalid secret" error (401)
- Ensure both backend and pipeline are reading from the same .env file
- Verify PIPELINE_SECRET matches in .env

### Archive not uploaded to S3
- The backend may fail to upload to S3 if AWS credentials aren't configured
- This is expected in local testing and won't prevent the pipeline from working
- Check backend logs for S3 errors

### Database update warnings
- Local testing with "output" as job ID will show warnings
- This is expected since "output" doesn't match "uid_jobindex" format
- Use a proper format like `testuser_5` to test database updates

## Production Environment

On the Metis cluster, the environment will be configured with:
```bash
PIPELINE_SECRET=<production-secret>
BACKEND_URL=https://igait.johncoxdev.com
```

And jobs will use proper IDs like `{firebase_uid}_{job_index}`.
