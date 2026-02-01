# iGait Backend - Queue-Based Microservices Architecture

Backend system for ASD gait analysis using Firebase RTDB queue-based workers.

> **Jump to your section:** This README is organized by developer role. Click the section that applies to you!

## 📑 Table of Contents

- **[Architecture Overview](#️-architecture)** - System design & processing stages
- **[Backend Developers](#-for-backend-developers)** - API server development
- **[Stage Developers](#️-for-stage-developers)** - Worker implementation guide
- **[Frontend Developers](#-for-frontend-developers)** - React app setup
- **[Email Developers](#-for-email-developers)** - Email template editing
- **[Local Development](#-local-development-docker)** - Docker setup
- **[Tech Stack](#-tech-stack)** - Technologies used

---

## 🏗️ Architecture

```
Backend (uploads)
    ↓
Firebase RTDB Queues
    ↓
Workers (poll & process)
    ↓
S3 Storage & Email Results
```

**7 Processing Stages:**
1. **Media Conversion** - Standardize video format (1080p, 60fps, H.264)
2. **Validity Check** - Verify person detection in videos
3. **Reframing** - Crop/adjust video framing
4. **Pose Estimation** - Extract body keypoints
5. **Cycle Detection** - Identify gait cycles
6. **Prediction** - ML inference for ASD classification
7. **Finalize** - Check results & send email notifications

---

## 🔧 For Backend Developers

**Structure:**
- `igait-backend/` - Main API server (Axum)
- `igait-lib/` - Shared library (storage, queues, email, types)

**Setup:**
```fish
cd igait-backend

# Set environment variables
cp .env.example .env
# Edit .env with your credentials

# Run locally
cargo run --release
```

**Key files:**
- `src/routes/upload.rs` - Handles video uploads, pushes to queue
- `src/routes/assistant.rs` - OpenAI assistant (optional)
- `src/helper/lib.rs` - App state (storage, email, Firebase clients)

**Making changes:**
1. Edit routes in `src/routes/`
2. Test with `cargo run`
3. Upload test: `./test_upload.sh` (requires test videos)

---

## ⚙️ For Stage Developers

**How stages work:**
Each stage is an independent worker that:
1. Polls Firebase RTDB queue (`queues/stage_N`)
2. Claims a job (distributed lock, 50min timeout)
3. Downloads inputs from S3
4. Processes data
5. Uploads outputs to S3
6. Pushes to next queue OR `queues/finalize` on failure

**Implementing your stage:**

```rust
// 1. Implement StageWorker trait
struct MyStageWorker { /* ... */ }

impl StageWorker for MyStageWorker {
    fn stage(&self) -> StageNumber {
        StageNumber::StageNYourStage  // Which stage you are
    }

    async fn process(&self, job: &QueueItem) -> ProcessingResult {
        // 2. Get input paths
        let front = job.input_front_video(self.stage());
        let side = job.input_side_video(self.stage());
        
        // 3. Download from S3
        let front_data = self.storage.download(&front).await?;
        
        // 4. YOUR PROCESSING LOGIC HERE
        let result = do_your_processing(front_data)?;
        
        // 5. Upload results to S3
        let output_key = job.output_front_video(self.stage());
        self.storage.upload(&output_key, result).await?;
        
        // 6. Return success
        Ok(ProcessingResult::Success)
    }
}

// 7. Run the worker
#[tokio::main]
async fn main() {
    run_stage_worker(MyStageWorker::new()).await;
}
```

**Testing your stage:**
```fish
cd igait-stages/igait-stage-N-yourname

# Build
cargo build --release

# Run (polls queues automatically)
cargo run --release

# Or with Docker
docker-compose up stageN
```

**File locations:**
- Each stage: `igait-stages/igait-stage-N-name/`
- Example: `igait-stages/igait-stage1-media-conversion/src/main.rs`

---

## 🎨 For Frontend Developers

**Structure:**
- `igait-web/` - React + Vite + TypeScript frontend

**Setup:**
```fish
cd igait-web

# Install dependencies (using bun)
bun install

# Run dev server
bun run dev
```

**Environment:**
Create `igait-web/.env` with:
```
VITE_FIREBASE_API_KEY=...
VITE_FIREBASE_AUTH_DOMAIN=...
VITE_BACKEND_URL=http://localhost:3000
```

**Key files:**
- `src/pages/` - Page components
- `src/components/` - Reusable components  
- `src/firebase.ts` - Firebase configuration

**Build for production:**
```fish
bun run build
```

---

## 📧 For Email Developers

**Location:** `igait-lib/src/microservice/email.rs`

**Templates available:**
1. **Submission Received** - Sent immediately after upload
2. **Prediction Success** - Sent when ASD score is calculated
3. **Processing Failure** - Sent if any stage fails

**Editing templates:**

```rust
// Find EmailTemplates in igait-lib/src/microservice/email.rs

pub struct EmailTemplates;

impl EmailTemplates {
    pub fn submission_received(job_id: &str, email: &str) -> EmailContent {
        EmailContent {
            subject: "Your Submission Has Been Received".to_string(),
            body_text: format!("Hello! ..."), // Plain text
            body_html: format!(r#"<html>...</html>"#), // HTML version
        }
    }
    
    pub fn prediction_success(...) -> EmailContent { /* ... */ }
    pub fn processing_failure(...) -> EmailContent { /* ... */ }
}
```

**Testing emails:**
1. Edit template in `email.rs`
2. Rebuild: `cargo build --release -p igait-backend`
3. Test upload to trigger email

**Where emails are sent:**
- Submission: `igait-backend/src/routes/upload.rs`
- Results: `igait-stages/igait-stage7-finalize/src/main.rs`

---

## 🐳 Local Development (Docker)

**Run everything:**
```fish
docker-compose up --build
```

**Services:**
- `backend` - :3000 - API server
- `stage1-7` - Workers (no exposed ports)

**Environment:** Copy `credentials/gcp-key.json` and configure `.env`

**See also:** [DOCKER.md](./DOCKER.md)

---

## 📦 Tech Stack
- Rust (backend + workers)
- React + TypeScript (frontend)
- Firebase (Auth + RTDB queues)
- AWS S3 (storage)
- AWS SES (email)
- Docker (deployment)

---

**Questions?** Check individual README files in subdirectories or ask the team! 🦊✨