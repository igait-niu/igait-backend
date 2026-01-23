# iGait Microservices Migration Plan

> 🦊 *A comprehensive guide to transform iGait from a monolithic architecture to stateless, scalable microservices~*

---

## 📋 Executive Summary

This document outlines the migration of iGait from its current monolithic architecture (tightly coupled backend + HPC pipeline) to a **stateless, event-driven microservices architecture** where each processing stage operates as an independent, horizontally-scalable service.

### Key Decisions ✅
| Decision | Choice |
|----------|--------|
| **State/Queue Management** | Firebase (Firestore) - already used for user data |
| **File Storage** | Google Cloud Storage (GCS) - GCP native |
| **Container Orchestration** | Kubernetes (GKE in production) |
| **Service API Language** | Rust (Axum) with shared `igait-lib` |
| **Processing Internals** | Flexible (Python, CUDA, FFmpeg, etc.) |
| **Target Cloud** | GCP (portable to any K8s environment) |

### Current Pain Points
- 🔗 **Tight HPC Coupling** - Backend depends on SSH/SCP to Metis cluster
- 📁 **Stateful File Handling** - Files saved locally before S3, creating state
- 🔄 **Synchronous Pipeline** - All stages run sequentially in one PBS job
- 📈 **Scaling Limitations** - Cannot scale individual stages independently
- 🐛 **Debugging Complexity** - Hard to isolate and debug stage failures

### Target Architecture Benefits
- ✅ **Horizontal Scaling** - Scale any stage independently based on load
- ✅ **Fault Isolation** - Stage failures don't cascade
- ✅ **Cloud-Native** - Containerized, runs on any K8s (GKE, EKS, AKS, etc.)
- ✅ **Stateless** - Any instance can handle any request
- ✅ **Observable** - Each service has clear inputs/outputs
- ✅ **Firebase-Powered** - Leverage existing Firebase infrastructure for job state

---

## 🏗️ Target Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              FRONTEND (Web App)                             │
└─────────────────────────────────┬───────────────────────────────────────────┘
                                  │ Upload Request
                                  ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                         BACKEND (Orchestrator)                              │
│  • Firebase Auth & Firestore Job Management                                 │
│  • GCS Upload Coordination                                                  │
│  • Stage Completion Webhooks                                                │
│  • Email Notifications                                                      │
│  • Job Status API                                                           │
└──────┬──────────────────────────────────────────────────────────────────────┘
       │                                      ▲
       │ Submit to Stage 1                    │ Webhook: Stage N Complete
       ▼                                      │
┌──────────────────┐    ┌──────────────────┐  │  ┌──────────────────┐
│  Stage 1 Service │───▶│  Stage 2 Service │──┼─▶│  Stage N Service │
│ Media Conversion │    │  Validity Check  │  │  │       ...        │
│   (Rust + FFmpeg)│    │ (Rust + Python)  │  │  │                  │
└────────┬─────────┘    └────────┬─────────┘  │  └────────┬─────────┘
         │                       │            │           │
         └───────────────────────┴────────────┴───────────┘
                                 │
              ┌──────────────────┼──────────────────┐
              ▼                  ▼                  ▼
    ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐
    │      GCS        │  │    Firestore    │  │  Firebase Auth  │
    │ (File Storage)  │  │  (Job State &   │  │    (Users)      │
    │                 │  │     Queue)      │  │                 │
    └─────────────────┘  └─────────────────┘  └─────────────────┘
```

### Why Firebase for Job Management?

Since we already use Firebase for authentication and user data, extending it for job queue/state management gives us:

1. **Real-time Updates** - Firestore listeners for instant job status updates to frontend
2. **No New Infrastructure** - Already have Firebase set up
3. **Atomic Transactions** - Firestore transactions for safe state transitions
4. **Built-in Security** - Firebase Security Rules protect job data
5. **Offline Support** - Frontend can show cached job status

---

## 🎯 Service Definitions

### Rust API Contract (igait-lib)

All microservices share a common Rust API layer via `igait-lib`. The crate will provide:

```rust
// igait-lib additions for microservices

/// Standard job request received from backend/previous stage
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StageJobRequest {
    pub job_id: String,           // e.g., "user123_5"
    pub user_id: String,          // e.g., "user123"
    pub stage: u8,                // Which stage this is (1-7)
    pub callback_url: String,     // Backend webhook URL
    pub input_keys: Vec<String>,  // GCS object keys for inputs
    pub metadata: JobMetadata,    // Optional stage-specific config
}

/// Standard response sent back to backend
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StageJobResult {
    pub job_id: String,
    pub stage: u8,
    pub status: StageResultStatus,
    pub output_keys: Vec<String>, // GCS object keys for outputs
    pub logs: String,
    pub duration_ms: u64,
    pub error: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum StageResultStatus {
    Success,
    Failed,
    Skipped,
}

/// Trait that all stage services must implement
#[async_trait]
pub trait StageService {
    /// Process a job - download from GCS, process, upload results
    async fn process(&self, request: StageJobRequest) -> Result<StageJobResult>;
    
    /// Health check
    async fn health(&self) -> HealthStatus;
    
    /// Get job status (if queued/processing)
    async fn job_status(&self, job_id: &str) -> Option<JobProgress>;
}
```

### Backend (Orchestrator) Service

**Responsibilities:**
- Accept initial upload requests from frontend
- Manage Firebase authentication and Firestore job documents
- Upload input files directly to GCS (no local storage)
- Create job document in Firestore with initial state
- Dispatch jobs to Stage 1 microservice
- Receive stage completion webhooks
- Update Firestore job document after each stage (real-time to frontend!)
- Trigger next stage or finalize job
- Send email notifications

**Firestore Job Document Schema:**
```typescript
// Collection: jobs/{job_id}
interface JobDocument {
  job_id: string;           // "user123_5"
  user_id: string;          // "user123"
  created_at: Timestamp;
  updated_at: Timestamp;
  
  // Patient metadata
  patient: {
    age: number;
    sex: string;
    height: string;
    weight: number;
    ethnicity: string;
  };
  
  // Current state
  status: 'submitted' | 'processing' | 'completed' | 'failed';
  current_stage: number;    // 0-7 (0 = uploaded, 7 = done)
  
  // Stage results (populated as stages complete)
  stages: {
    [stageNum: string]: {
      status: 'pending' | 'processing' | 'success' | 'failed' | 'skipped';
      started_at?: Timestamp;
      completed_at?: Timestamp;
      duration_ms?: number;
      output_keys?: string[];
      error?: string;
    }
  };
  
  // Final result (populated after stage 7)
  result?: {
    score: number;          // ASD probability 0-1
    classification: string; // "ASD" or "NO ASD"
    archive_key: string;    // GCS key for results.zip
  };
  
  // Notification tracking
  email: string;
  email_sent: boolean;
}
```

**API Endpoints:**

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/v1/upload` | POST | Initial job submission |
| `/api/v1/jobs/{job_id}` | GET | Get job status (or use Firestore directly) |
| `/api/v1/webhook/stage/{stage_num}` | POST | Stage completion callback |
| `/api/v1/assistant` | WS | AI assistant (unchanged) |

### Stage Microservice Template

Each stage service is a **Rust Axum web server** that wraps stage-specific processing logic.

**Standard API Endpoints (all stages):**

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/submit` | POST | Submit a job (accepts `StageJobRequest`) |
| `/health` | GET | Health check (liveness/readiness) |
| `/jobs/{job_id}` | GET | Get job status |
| `/metrics` | GET | Prometheus metrics |

**Architecture of Each Stage Service:**
```
┌─────────────────────────────────────────────────────┐
│              Stage N Microservice                   │
│  ┌─────────────────────────────────────────────┐   │
│  │         Rust API Layer (Axum)               │   │
│  │  • POST /submit                             │   │
│  │  • GET /health                              │   │
│  │  • GET /jobs/{id}                           │   │
│  └─────────────────┬───────────────────────────┘   │
│                    │                               │
│  ┌─────────────────▼───────────────────────────┐   │
│  │         Processing Core                      │   │
│  │  (FFmpeg, Python, CUDA, etc.)               │   │
│  └─────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────┘
```

---

## 📦 Microservice Specifications

> All services have a **Rust (Axum) API layer** that implements the `StageService` trait from `igait-lib`.
> The processing internals can use any technology appropriate for the task.

### Stage 1: Media Conversion Service

**Purpose:** Standardize video format (1080p, 60fps, H.264)

**Stack:**
- **API:** Rust (Axum) + `igait-lib`
- **Processing:** FFmpeg (called via `tokio::process::Command`)
- **Base Image:** `rust:alpine` + FFmpeg

**Resource Requirements:**
- CPU: 2-4 cores (FFmpeg is CPU-intensive)
- Memory: 2-4 GB
- GPU: Not required

**Dockerfile:**
```dockerfile
FROM rust:1.75-alpine AS builder
WORKDIR /app
COPY . .
RUN cargo build --release -p stage1-service

FROM alpine:latest
RUN apk add --no-cache ffmpeg ca-certificates
COPY --from=builder /app/target/release/stage1-service /usr/local/bin/
ENV RUST_LOG=info
EXPOSE 8080
CMD ["stage1-service"]
```

---

### Stage 2: Validity Check Service

**Purpose:** Verify person is detectable in video frames

**Stack:**
- **API:** Rust (Axum) + `igait-lib`
- **Processing:** Python subprocess with OpenCV/MediaPipe
- **Base Image:** `rust:slim` + Python 3.12

**Resource Requirements:**
- CPU: 2 cores
- Memory: 2 GB
- GPU: Optional (CPU detection works fine)

---

### Stage 3: Reframing Service

**Purpose:** Adjust video framing/cropping based on detected person

**Stack:**
- **API:** Rust (Axum) + `igait-lib`
- **Processing:** FFmpeg crop filters
- **Base Image:** `rust:alpine` + FFmpeg

**Resource Requirements:**
- CPU: 2 cores
- Memory: 2 GB

---

### Stage 4: Pose Estimation Service

**Purpose:** Extract body keypoints using OpenPose

**Stack:**
- **API:** Rust (Axum) + `igait-lib`
- **Processing:** OpenPose (or MediaPipe as lighter alternative)
- **Base Image:** `nvidia/cuda:12.0-runtime` + Rust + OpenPose

**Resource Requirements:**
- CPU: 4 cores
- Memory: 8 GB
- GPU: **Required** (NVIDIA with CUDA) - or use MediaPipe for CPU-only

**Notes:**
- Most resource-intensive stage
- Consider GKE GPU node pools with auto-scaling
- Batch processing window for cost efficiency

---

### Stage 5: Cycle Detection Service

**Purpose:** Identify gait cycles from pose keypoint data

**Stack:**
- **API:** Rust (Axum) + `igait-lib`
- **Processing:** Pure Rust or Python (NumPy/SciPy)
- **Base Image:** `rust:alpine` (or + Python if needed)

**Resource Requirements:**
- CPU: 1-2 cores
- Memory: 1 GB
- GPU: Not required

---

### Stage 6: Prediction Service

**Purpose:** ML inference for ASD classification

**Stack:**
- **API:** Rust (Axum) + `igait-lib`
- **Processing:** Python + TensorFlow/PyTorch model
- **Base Image:** `rust:slim` + Python 3.12 + ML dependencies

**Resource Requirements:**
- CPU: 2-4 cores (CPU inference)
- Memory: 4 GB
- GPU: Optional (speeds up inference but not required)

---

### Stage 7: Archive Service

**Purpose:** Package all results into ZIP archive

**Stack:**
- **API:** Rust (Axum) + `igait-lib`
- **Processing:** Pure Rust (`zip` crate)
- **Base Image:** `rust:alpine`

**Resource Requirements:**
- CPU: 1 core
- Memory: 1 GB

---

## 🗄️ Google Cloud Storage (GCS) Structure

```
gs://igait-storage/
├── jobs/
│   └── {user_id}_{job_index}/
│       ├── stage_0/           # Original uploads (from backend)
│       │   ├── front.mp4
│       │   └── side.mp4
│       ├── stage_1/           # Media conversion output
│       │   ├── front.mp4
│       │   └── side.mp4
│       ├── stage_2/           # Validity check output
│       │   └── validation.json
│       ├── stage_3/           # Reframing output
│       │   ├── front.mp4
│       │   └── side.mp4
│       ├── stage_4/           # Pose estimation output
│       │   ├── front_keypoints.json
│       │   ├── side_keypoints.json
│       │   └── overlays/
│       │       ├── front_overlay.mp4
│       │       └── side_overlay.mp4
│       ├── stage_5/           # Cycle detection output
│       │   └── cycles.json
│       ├── stage_6/           # Prediction output
│       │   └── prediction.json
│       └── stage_7/           # Final archive
│           └── results.zip
├── models/                    # ML models (versioned)
│   ├── asd_classifier_v1/
│   └── asd_classifier_v2/
└── logs/                      # Archived job logs (optional)
```

### GCS Access Pattern

Each microservice uses a **GCP Service Account** with minimal permissions:

| Service | GCS Permissions |
|---------|-----------------|
| Backend | `storage.objects.create` on `jobs/*/stage_0/*` |
| Stage 1 | Read `stage_0`, Write `stage_1` |
| Stage 2 | Read `stage_1`, Write `stage_2` |
| Stage N | Read `stage_{N-1}`, Write `stage_N` |
| Stage 7 | Read all stages, Write `stage_7` |

This follows the **principle of least privilege** - each service can only access what it needs.

---

## 🔄 Event Flow Diagram

```
┌──────────┐       ┌─────────┐       ┌────────┐       ┌─────────────┐
│ Frontend │──────▶│ Backend │──────▶│   S3   │◀─────▶│ All Stages  │
└──────────┘       └────┬────┘       └────────┘       └──────┬──────┘
                        │                                     │
                        │  1. Upload to S3 (stage_0)          │
                        │  2. Create job in Firebase          │
                        │  3. POST to Stage 1 /submit         │
                        │◀────────────────────────────────────┤
                        │  4. Stage 1 pulls from S3           │
                        │  5. Stage 1 processes               │
                        │  6. Stage 1 uploads to S3 (stage_1) │
                        │  7. Stage 1 calls webhook           │
                        │                                     │
                        │  8. Backend updates Firebase        │
                        │  9. Backend POSTs to Stage 2        │
                        │◀────────────────────────────────────┤
                        │       ... (repeat for each stage)   │
                        │                                     │
                        │  N. Final webhook from Stage 7      │
                        │  N+1. Backend sends completion email│
                        ▼                                     │
```

---

## 🛠️ Implementation Phases

> No rigid timelines here - we move at whatever pace feels right! 🦊

### Phase 1: Foundation

**Objective:** Set up infrastructure and extend `igait-lib`

- [ ] **1.1** Extend `igait-lib` with microservice types:
  - `StageJobRequest`, `StageJobResult`, `StageResultStatus`
  - `StageService` trait
  - GCS client utilities (upload/download helpers)
  - Webhook callback client
- [ ] **1.2** Create GCS bucket with new structure
- [ ] **1.3** Set up Firestore collections (`jobs`, with security rules)
- [ ] **1.4** Create microservice template crate (`igait-stage-template`)
  - Axum server boilerplate
  - Health check endpoint
  - Job submission endpoint
  - Prometheus metrics
- [ ] **1.5** Local dev environment (docker-compose with GCS emulator)
- [ ] **1.6** CI/CD pipeline for building/pushing images to Artifact Registry

**Done when:** We can spin up a dummy stage service locally that receives a job, "processes" it (no-op), and calls the webhook.

---

### Phase 2: Backend Refactor

**Objective:** Transform backend into stateless orchestrator

- [ ] **2.1** Replace S3 with GCS client
- [ ] **2.2** Replace Firebase RTDB with Firestore for jobs
- [ ] **2.3** Remove local file storage - stream directly to GCS
- [ ] **2.4** Remove Metis/SSH integration entirely
- [ ] **2.5** Implement webhook endpoints for each stage
- [ ] **2.6** Create stage dispatcher (HTTP client to call stage services)
- [ ] **2.7** Update email notifications to trigger on Firestore updates
- [ ] **2.8** Add retry logic with exponential backoff for stage calls

**Done when:** Backend can accept an upload, store it in GCS, create a Firestore job doc, and call Stage 1.

---

### Phase 3: First Stage (Reference Implementation)

**Objective:** Build Stage 1 as the template for all others

- [ ] **3.1** Create `igait-stage1-media-conversion` crate
- [ ] **3.2** Port FFmpeg logic from `s1_media_conversion.rs`
- [ ] **3.3** Implement full `StageService` trait
- [ ] **3.4** GCS download → FFmpeg → GCS upload flow
- [ ] **3.5** Webhook callback on completion
- [ ] **3.6** Comprehensive error handling + logging
- [ ] **3.7** Docker multi-stage build
- [ ] **3.8** Integration test with backend

**Done when:** Full flow works: Upload → Backend → Stage 1 → Webhook → Firestore updated.

---

### Phase 4: Remaining Stages

**Objective:** Build all remaining microservices

Each stage follows the same pattern established in Phase 3:

- [ ] **4.1** Stage 2 - Validity Check (Rust API + Python/OpenCV internals)
- [ ] **4.2** Stage 3 - Reframing (Rust API + FFmpeg)
- [ ] **4.3** Stage 4 - Pose Estimation (Rust API + OpenPose/MediaPipe)
  - GPU container configuration
  - Consider MediaPipe for CPU fallback
- [ ] **4.4** Stage 5 - Cycle Detection (Rust API + Rust/Python internals)
- [ ] **4.5** Stage 6 - Prediction (Rust API + Python ML model)
- [ ] **4.6** Stage 7 - Archive (Pure Rust)

**Done when:** Full 7-stage pipeline runs end-to-end in docker-compose.

---

### Phase 5: GKE Production Setup

**Objective:** Deploy to Google Kubernetes Engine

- [ ] **5.1** Create GKE cluster (Autopilot recommended for simplicity)
- [ ] **5.2** Configure GPU node pool for Stage 4 (if using OpenPose)
- [ ] **5.3** Create Kubernetes manifests:
  - Deployments for each stage
  - Services (ClusterIP for internal, LoadBalancer for backend)
  - HorizontalPodAutoscalers
  - ConfigMaps and Secrets
- [ ] **5.4** Set up Workload Identity for GCS access
- [ ] **5.5** Configure Cloud Armor / IAP for backend protection
- [ ] **5.6** Set up Cloud Monitoring dashboards
- [ ] **5.7** Load testing with realistic traffic

**Done when:** Pipeline runs successfully on GKE with auto-scaling.

---

### Phase 6: Cutover

**Objective:** Migrate production traffic

- [ ] **6.1** Run shadow mode (both systems process same jobs)
- [ ] **6.2** Compare results between old and new pipelines
- [ ] **6.3** Gradual traffic shift
- [ ] **6.4** Decommission Metis pipeline
- [ ] **6.5** Update all documentation
- [ ] **6.6** Celebrate! 🎉🦊

**Done when:** Old system is off, new system handles all traffic.

---

## 🐳 Docker Compose (Local Development)

```yaml
version: '3.8'

services:
  # GCS emulator for local development
  gcs-emulator:
    image: fsouza/fake-gcs-server
    ports:
      - "4443:4443"
    command: ["-scheme", "http", "-port", "4443"]
    volumes:
      - gcs-data:/data

  # Firebase emulator (Firestore + Auth)
  firebase-emulator:
    image: andreysenov/firebase-tools
    ports:
      - "4000:4000"   # Emulator UI
      - "8080:8080"   # Firestore
      - "9099:9099"   # Auth
    command: ["firebase", "emulators:start", "--only", "firestore,auth"]

  backend:
    build: ./igait-backend
    ports:
      - "3000:3000"
    environment:
      - GCS_ENDPOINT=http://gcs-emulator:4443
      - GCS_BUCKET=igait-storage
      - FIRESTORE_EMULATOR_HOST=firebase-emulator:8080
      - FIREBASE_AUTH_EMULATOR_HOST=firebase-emulator:9099
      - STAGE_1_URL=http://stage1:8080
      - STAGE_2_URL=http://stage2:8080
      - STAGE_3_URL=http://stage3:8080
      - STAGE_4_URL=http://stage4:8080
      - STAGE_5_URL=http://stage5:8080
      - STAGE_6_URL=http://stage6:8080
      - STAGE_7_URL=http://stage7:8080
    depends_on:
      - gcs-emulator
      - firebase-emulator
      - stage1

  stage1:
    build: ./services/stage1-media-conversion
    ports:
      - "8081:8080"
    environment:
      - GCS_ENDPOINT=http://gcs-emulator:4443
      - GCS_BUCKET=igait-storage
      - RUST_LOG=info

  stage2:
    build: ./services/stage2-validity-check
    ports:
      - "8082:8080"
    environment:
      - GCS_ENDPOINT=http://gcs-emulator:4443
      - GCS_BUCKET=igait-storage

  stage3:
    build: ./services/stage3-reframing
    ports:
      - "8083:8080"
    environment:
      - GCS_ENDPOINT=http://gcs-emulator:4443
      - GCS_BUCKET=igait-storage

  stage4:
    build: ./services/stage4-pose-estimation
    ports:
      - "8084:8080"
    environment:
      - GCS_ENDPOINT=http://gcs-emulator:4443
      - GCS_BUCKET=igait-storage
    # Uncomment for GPU support (requires nvidia-docker)
    # deploy:
    #   resources:
    #     reservations:
    #       devices:
    #         - capabilities: [gpu]

  stage5:
    build: ./services/stage5-cycle-detection
    ports:
      - "8085:8080"
    environment:
      - GCS_ENDPOINT=http://gcs-emulator:4443
      - GCS_BUCKET=igait-storage

  stage6:
    build: ./services/stage6-prediction
    ports:
      - "8086:8080"
    environment:
      - GCS_ENDPOINT=http://gcs-emulator:4443
      - GCS_BUCKET=igait-storage

  stage7:
    build: ./services/stage7-archive
    ports:
      - "8087:8080"
    environment:
      - GCS_ENDPOINT=http://gcs-emulator:4443
      - GCS_BUCKET=igait-storage

volumes:
  gcs-data:
```

---

## 📊 Kubernetes Deployment (Production)

```yaml
# Example: Stage 1 Deployment
apiVersion: apps/v1
kind: Deployment
metadata:
  name: stage1-media-conversion
spec:
  replicas: 3
  selector:
    matchLabels:
      app: stage1
  template:
    metadata:
      labels:
        app: stage1
    spec:
      containers:
      - name: stage1
        image: ghcr.io/igait-niu/stage1:latest
        ports:
        - containerPort: 8080
        resources:
          requests:
            cpu: "2"
            memory: "2Gi"
          limits:
            cpu: "4"
            memory: "4Gi"
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 10
          periodSeconds: 30
        readinessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 10
---
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: stage1-hpa
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: stage1-media-conversion
  minReplicas: 2
  maxReplicas: 10
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
```

---

## 🔒 Security Considerations

### Service-to-Service Authentication
- **Option A:** JWT tokens signed by backend for stage-to-backend callbacks
- **Option B:** mTLS between services (if using service mesh)
- **Option C:** API keys with HMAC signatures (simplest)

### S3 Access
- IAM roles for services (preferred in AWS)
- Presigned URLs for temporary access
- Bucket policies to restrict access per service

### Secrets Management
- Use Kubernetes Secrets or AWS Secrets Manager
- Never commit credentials to code
- Rotate secrets regularly

---

## 🧪 Testing Strategy

### Unit Tests
- Each service has its own unit test suite
- Mock S3 and external dependencies

### Integration Tests
- Test service communication
- Use LocalStack for S3

### End-to-End Tests
- Full pipeline run with test videos
- Compare output with known-good results

### Load Tests
- Simulate concurrent job submissions
- Test auto-scaling behavior

---

## 📈 Monitoring & Observability

### Metrics (Prometheus)
- `jobs_submitted_total` - Counter
- `jobs_completed_total` - Counter (by stage, status)
- `job_duration_seconds` - Histogram (by stage)
- `queue_depth` - Gauge (per service)

### Logging (Structured JSON)
```json
{
  "timestamp": "2026-01-23T10:30:00Z",
  "level": "info",
  "service": "stage1",
  "job_id": "user123_5",
  "message": "Processing started",
  "trace_id": "abc123"
}
```

### Tracing
- Propagate trace IDs through all services
- Use OpenTelemetry for vendor-neutral instrumentation

---

## ❓ Open Questions & Decisions Needed

1. **Queue Technology:** Redis vs RabbitMQ vs SQS?
2. **GPU Strategy:** Cloud GPUs vs dedicated hardware for Stage 4?
3. **Service Language:** Rust for all services or Python for ML stages?
4. **Deployment Platform:** Kubernetes vs ECS vs Cloud Run?
5. **CI/CD:** GitHub Actions vs GitLab CI vs Jenkins?
6. **Monitoring Stack:** Prometheus+Grafana vs Datadog vs CloudWatch?

---

## 📚 References

- [12-Factor App Methodology](https://12factor.net/)
- [Microservices Patterns](https://microservices.io/patterns/)
- [AWS Well-Architected Framework](https://aws.amazon.com/architecture/well-architected/)
- [Kubernetes Documentation](https://kubernetes.io/docs/)

---

*Document created: January 2026*  
*Last updated: January 2026*

🦊✨ *We can do this together~! Let me know if you'd like to discuss any section in more detail!*
