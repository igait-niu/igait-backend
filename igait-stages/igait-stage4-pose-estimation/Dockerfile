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
COPY igait-stages/igait-stage1-media-conversion ./igait-stages/igait-stage1-media-conversion
COPY igait-stages/igait-stage2-validity-check ./igait-stages/igait-stage2-validity-check
COPY igait-stages/igait-stage3-reframing ./igait-stages/igait-stage3-reframing
COPY igait-stages/igait-stage4-pose-estimation ./igait-stages/igait-stage4-pose-estimation
COPY igait-stages/igait-stage5-cycle-detection ./igait-stages/igait-stage5-cycle-detection
COPY igait-stages/igait-stage6-prediction ./igait-stages/igait-stage6-prediction

# Build the stage service
RUN cargo build --release -p igait-stage4-pose-estimation

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
