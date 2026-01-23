# 🐳 Docker Development Setup

This directory contains Docker configurations for local development of the iGait microservices architecture.

## 📋 Prerequisites

1. **Docker & Docker Compose** installed
2. **Firebase credentials** configured (service account JSON)
3. **Environment Variables** configured (see below)

## 🚀 Quick Start

### 1. Set Up Credentials

Create a `credentials/` directory and place your Firebase service account JSON:

```bash
mkdir -p credentials
# Copy your Firebase service account key file here
cp ~/path/to/your-firebase-key.json credentials/gcp-key.json
```

### 2. Configure Environment Variables

Copy the example env file and fill in your values:

```bash
cp .env.example .env
# Edit .env with your actual credentials
```

Required variables:
- `FIREBASE_ACCESS_KEY` - Firebase RTDB secret
- `OPENAI_API_KEY` - OpenAI API key
- `OPENAI_ASSISTANT_ID` - Your assistant ID
- `OPENAI_VECTOR_STORE_ID` - Vector store ID
- `AWS_ACCESS_KEY_ID` / `AWS_SECRET_ACCESS_KEY` - For SES emails

### 3. Build and Run

```bash
# Build all services
docker-compose build

# Start all services
docker-compose up

# Or build + start in one command
docker-compose up --build
```

### 4. Test the Backend

The backend will be available at `http://localhost:3000`

```bash
# Health check (once implemented)
curl http://localhost:3000/health

# Upload endpoint
curl -X POST http://localhost:3000/api/v1/upload \
  -F "uid=test_user" \
  -F "age=25" \
  -F "ethnicity=Test" \
  -F "sex=M" \
  -F "height=5'10\"" \
  -F "weight=170" \
  -F "email=test@example.com" \
  -F "fileuploadfront=@front.mp4" \
  -F "fileuploadside=@side.mp4"
```

## 🏗️ Service Architecture

```
┌─────────────────────────────────────────┐
│  Backend (port 3000)                    │
│  - Job orchestration                    │
│  - Firebase Storage upload              │
│  - Stage dispatching                    │
│  - Email notifications                  │
└────────────┬────────────────────────────┘
             │
    ┌────────┴────────┐
    │                 │
┌───▼───┐  ┌──────▼──────┐  ... ┌──────▼──────┐
│Stage 1│─▶│   Stage 2   │─────▶│   Stage 6   │
│ 8080  │  │    8080     │      │    8080     │
└───────┘  └─────────────┘      └─────────────┘
```

Each stage:
- Receives jobs via POST `/submit`
- Downloads from Firebase Storage
- Processes the data
- Uploads results to Firebase Storage
- Calls backend webhook on completion

## 📦 Individual Services

### Backend (`igait-backend/`)
- **Image**: `debian:bookworm-slim` + Rust binary
- **Port**: 3000
- **Dependencies**: libssl, ca-certificates

### Stage 1: Media Conversion (`igait-media-conversion-stage/`)
- **Image**: `alpine` + FFmpeg
- **Purpose**: Convert videos to standard format (1080p, 60fps, H.264)

### Stage 2: Validity Check (`igait-validity-check-stage/`)
- **Image**: `debian` + Python + OpenCV + MediaPipe
- **Purpose**: Verify person is detectable in videos

### Stage 3: Reframing (`igait-reframing-stage/`)
- **Image**: `alpine` + FFmpeg
- **Purpose**: Crop/adjust video framing

### Stage 4: Pose Estimation (`igait-pose-estimation-stage/`)
- **Image**: `debian` + Python + MediaPipe
- **Purpose**: Extract body keypoints (CPU-based)
- **Note**: For GPU/OpenPose, use `nvidia/cuda` base image

### Stage 5: Cycle Detection (`igait-cycle-detection-stage/`)
- **Image**: `alpine` + Rust
- **Purpose**: Identify gait cycles from keypoints

### Stage 6: Prediction (`igait-prediction-stage/`)
- **Image**: `debian` + Python + TensorFlow
- **Purpose**: ML inference for ASD classification

## 🛠️ Development Tips

### View Logs

```bash
# All services
docker-compose logs -f

# Specific service
docker-compose logs -f backend
docker-compose logs -f stage1
```

### Rebuild After Code Changes

```bash
# Rebuild specific service
docker-compose build backend

# Rebuild and restart
docker-compose up --build backend
```

### Stop Services

```bash
# Stop all
docker-compose down

# Stop and remove volumes
docker-compose down -v
```

### Debug a Specific Stage

```bash
# Run just one stage
docker-compose up stage1

# Shell into a running container
docker-compose exec stage1 sh
```

## 🔧 Troubleshooting

### Port Conflicts
If port 3000 is in use:
```bash
# Change PORT in .env
PORT=3001

# Or specify in docker-compose
ports:
  - "3001:3000"
```

### Credential Issues
```bash
# Verify credentials file exists
ls -la credentials/gcp-key.json

# Check it's mounted correctly
docker-compose exec backend ls -la /app/credentials/
```

### Build Failures
```bash
# Clean build cache
docker-compose build --no-cache

# Check Rust version in Dockerfile matches
docker run rust:1.75-slim rustc --version
```

## 📝 Notes

- The `docker-compose.yml` file builds all services from source for local development
- Stages communicate via internal `igait-network` bridge network
- All services share the same Firebase credentials mounted read-only
- For production deployment, use Kubernetes manifests (see deployment docs)

## 🎯 Next Steps

1. Implement actual processing logic in each stage
2. Add health check endpoints (`/health`)
3. Add Prometheus metrics endpoints (`/metrics`)
4. Create Kubernetes deployment manifests
5. Set up CI/CD pipeline for automated builds

---

*Happy developing! 🦊✨*
