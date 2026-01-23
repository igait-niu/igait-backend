# Multi-stage build for Stage 4: Pose Estimation
# Note: This uses CPU-based MediaPipe. For GPU/OpenPose, use nvidia/cuda base image
FROM rust:1.93-slim as builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    python3 \
    python3-pip \
    && rm -rf /var/lib/apt/lists/*

# Copy workspace files
COPY Cargo.toml ./
COPY igait-lib ./igait-lib
COPY igait-backend ./igait-backend
COPY igait-media-conversion-stage ./igait-media-conversion-stage
COPY igait-validity-check-stage ./igait-validity-check-stage
COPY igait-reframing-stage ./igait-reframing-stage
COPY igait-pose-estimation-stage ./igait-pose-estimation-stage
COPY igait-cycle-detection-stage ./igait-cycle-detection-stage
COPY igait-prediction-stage ./igait-prediction-stage

# Build the stage service
RUN cargo build --release -p igait-pose-estimation-stage

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    python3 \
    python3-pip \
    python3-opencv \
    && rm -rf /var/lib/apt/lists/*

# Install Python dependencies (MediaPipe for CPU-based pose estimation)
RUN pip3 install --no-cache-dir --break-system-packages mediapipe numpy opencv-python

# Copy the binary
COPY --from=builder /app/target/release/igait-pose-estimation-stage /usr/local/bin/stage-service

ENV RUST_LOG=info
ENV PORT=8080

EXPOSE 8080

CMD ["stage-service"]
