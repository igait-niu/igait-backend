# iGait Technologies & Architecture Reference

> 🦊 *This document helps contextualize the iGait project for future development sessions~*

## 📦 Project Structure

iGait is a **Rust-based monorepo** with three main workspace members:

| Crate | Purpose |
|-------|---------|
| `igait-backend` | Web server handling uploads, job management, Firebase/GCS integration |
| `igait-pipeline` | Gait analysis pipeline with 7 processing stages |
| `igait-lib` | Shared types, data structures, and microservice utilities |

---

## 🎯 Migration Target (In Progress)

We're migrating to a **stateless microservices architecture**! See [MICROSERVICES_MIGRATION.md](../MICROSERVICES_MIGRATION.md) for full details.

| Decision | Choice |
|----------|--------|
| **State/Queue** | Firebase (Firestore) |
| **File Storage** | Firebase Storage (`network-technology-project.firebasestorage.app`) |
| **Orchestration** | Kubernetes (GKE) |
| **Service APIs** | Rust (Axum) via `igait-lib` |
| **Target Cloud** | GCP |

---

## 🛠️ Core Technologies

### Language & Runtime
- **Rust** (Edition 2021) - Primary language for backend and pipeline
- **Python 3.12** - ML inference in Stage 6
- **CUDA/MPI** - GPU acceleration for pose estimation (Stage 1 assets)

### Web Framework
- **Axum 0.7** - Async web framework with:
  - Multipart file upload support
  - WebSocket support (`/assistant` routes)
  - Tower middleware integration
  - 500MB body limit for video uploads

### Async Runtime
- **Tokio** (full features) - Async runtime for all async operations

### Authentication & Authorization
- **Firebase Auth** - User authentication via `firebase-auth` crate
- **Firebase Realtime Database** - User/job data storage via `firebase-rs` (migrating to Firestore)
- **Shared secrets** - Pipeline submission verification (`X-Pipeline-Secret` header)

### Cloud Services (Current: AWS → Migrating to GCP)
- **S3/GCS** - File storage for:
  - Input videos: `data/{uid}/inputs/{uid};{job_id}/`
  - Results: `results/{job_id}/results.zip`
- **SES v2** (`aws-sdk-sesv2`) - Email notifications (welcome, success, failure)

### AI/ML Integration
- **OpenAI API** (`async-openai`) - AI assistant functionality
- **Custom ML Model** - ASD prediction (TensorFlow/PyTorch in Stage 6)

### HPC Integration
- **Metis (NIU HPC Cluster)** - Pipeline execution environment
- **PBS (Portable Batch System)** - Job scheduling via `qsub`
- **OpenSSH** (`openssh` crate) - SSH/SCP for file transfer and remote execution
- **Singularity/Apptainer** - Container runtime for OpenPose

### Media Processing
- **FFmpeg** - Video standardization (Stage 1)
  - Converts to 1080p @ 60fps, H.264, AAC audio
- **OpenPose** (Singularity container) - Pose estimation (Stage 4)

---

## 🔄 Current Data Flow

```
┌─────────────┐     ┌──────────────┐     ┌─────────────┐
│   Frontend  │────▶│   Backend    │────▶│   Firebase  │
│  (Web App)  │     │  (Axum API)  │     │  (Auth/DB)  │
└─────────────┘     └──────┬───────┘     └─────────────┘
                          │
                    ┌─────▼─────┐
                    │    S3     │ (Input videos)
                    └─────┬─────┘
                          │ SCP
                    ┌─────▼─────┐
                    │   Metis   │ (HPC Cluster)
                    │  Pipeline │
                    └─────┬─────┘
                          │ HTTP POST
                    ┌─────▼─────┐
                    │  Backend  │ (Update DB, S3)
                    └───────────┘
```

---

## 🧪 Pipeline Stages

| Stage | Name | Technology | Description |
|-------|------|------------|-------------|
| 1 | Media Conversion | FFmpeg | Standardize video format (1080p, 60fps, H.264) |
| 2 | Validity Check | Custom | Verify person is detectable in video |
| 3 | Reframing | Custom | Adjust video framing |
| 4 | Pose Estimation | OpenPose (Singularity) | Extract body keypoints |
| 5 | Cycle Detection | Custom | Identify gait cycles |
| 6 | Prediction | Python/ML | ASD classification (threshold: 0.5) |
| 7 | Archive | Custom | Package results as ZIP |

---

## 📊 Key Data Structures

### `Output` (igait-lib)
```rust
pub struct Output {
    pub canonical_paths: CanonicalPaths,
    pub stages: Stages,
    pub result: Result<f64, String>,  // ASD score or error
    pub skip_to_stage: Option<u8>,
}
```

### `Job` (Backend)
```rust
pub struct Job {
    pub age: i16,
    pub ethnicity: String,
    pub sex: char,
    pub height: String,
    pub weight: i16,
    pub status: JobStatus,
    pub timestamp: SystemTime,
    pub email: String,
}
```

### `JobStatusCode`
- `Submitting` - Files being uploaded
- `SubmissionErr` - Upload failed
- `Queue` - Waiting for processing
- `Processing` - Pipeline running
- `InferenceErr` - Pipeline failed
- `Complete` - Analysis finished

---

## 🌐 API Endpoints

| Route | Method | Description |
|-------|--------|-------------|
| `/api/v1/upload` | POST | Submit new gait analysis job |
| `/api/v1/contribute` | POST | Contribute data for research |
| `/api/v1/assistant` | ANY | AI assistant WebSocket |
| `/api/v1/assistant_proxied` | ANY | Proxied assistant endpoint |
| `/api/v1/pipeline/submit` | POST | Receive pipeline results (internal) |

---

## 🐳 Current Deployment

- **Docker Compose** with three services:
  - `backend` - igait-backend (port 3000)
  - `frontend` - igait-web (port 4173)
  - `socials-test` - Test service (port 4000)
- **Nix Flakes** - Reproducible builds available

---

## 📁 Important Paths

### Backend
- Inputs: `inputs/{uid};{job_id}/`
- Outputs: `outputs/{uid}_{job_id}/`

### Metis (HPC)
- Pipeline: `/lstr/sahara/zwlab/jw/igait-pipeline/`
- Inputs: `/lstr/sahara/zwlab/data/inputs/`
- PBS Script: `/lstr/sahara/zwlab/jw/igait-pipeline/igait-pipeline/pipeline.pbs`
- FFmpeg: `/lstr/sahara/zwlab/ffmpeg/bin/ffmpeg`
- OpenPose SIF: `/lstr/sahara/zwlab/jw/igait-pipeline/igait-openpose/igait-openpose.sif`

---

## 🔐 Environment Variables

| Variable | Purpose |
|----------|---------|
| `AWS_ACCESS_KEY_ID` | S3/SES authentication |
| `AWS_SECRET_ACCESS_KEY` | S3/SES authentication |
| `FIREBASE_ACCESS_KEY` | Firebase Realtime DB access |
| `OPENAI_ASSISTANT_ID` | AI assistant configuration |
| `PIPELINE_SECRET` | Pipeline submission authentication |
| `PORT` | Server port (default: 3000) |

---

## 📝 Notes for Future Sessions

- The pipeline currently runs on Metis via PBS job submission
- Files are transferred via SCP, which creates tight coupling
- The backend saves files locally AND to S3 (redundant)
- Email notifications are controlled by `DISABLE_RESULT_EMAIL` flag
- ASD classification threshold is hardcoded at 0.5

---

*Last updated: January 2026* 🦊✨
