FROM rust:1.93-alpine AS builder

RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static pkgconfig

WORKDIR /app

# Copy Cargo manifests for dependency caching
COPY igait-lib/Cargo.toml ./igait-lib/Cargo.toml
COPY igait-stages/igait-stage2-validity-check/Cargo.toml ./igait-stages/igait-stage2-validity-check/Cargo.toml

# Create dummy source files to cache dependencies
RUN mkdir -p igait-lib/src && echo "pub fn dummy() {}" > igait-lib/src/lib.rs
RUN mkdir -p igait-stages/igait-stage2-validity-check/src && echo "fn main() {}" > igait-stages/igait-stage2-validity-check/src/main.rs

# Build dependencies only (this layer will be cached)
WORKDIR /app/igait-stages/igait-stage2-validity-check
RUN cargo build --release 2>/dev/null || true

# Now copy actual source files
WORKDIR /app
COPY igait-lib ./igait-lib
COPY igait-stages/igait-stage2-validity-check/Cargo.toml ./igait-stages/igait-stage2-validity-check/Cargo.toml
COPY igait-stages/igait-stage2-validity-check/src ./igait-stages/igait-stage2-validity-check/src

# Build with actual source
WORKDIR /app/igait-stages/igait-stage2-validity-check
# Clean the dummy build artifacts first
RUN cargo clean --release
RUN cargo build --release

# ── Runtime stage ────────────────────────────────────────────────
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    python3 \
    python3-pip \
    python3-venv \
    python3-dev \
    python3-opencv \
    git \
    wget \
    libgl1-mesa-glx \
    libglib2.0-0 \
    libgomp1 \
    && rm -rf /var/lib/apt/lists/*

# Install CPU-only PyTorch and ML dependencies
RUN pip3 install --no-cache-dir --break-system-packages \
    torch torchvision --index-url https://download.pytorch.org/whl/cpu

RUN pip3 install --no-cache-dir --break-system-packages \
    pytorchvideo \
    scipy \
    numpy \
    "opencv-python>=4.1.1" \
    "pillow>=10.3.0" \
    "PyYAML>=5.3.1" \
    "tqdm>=4.66.3" \
    "matplotlib>=3.3" \
    seaborn \
    pandas \
    ultralytics

# Copy the detection pipeline
COPY igait-stages/igait-stage2-validity-check/igait-human-gait-detection /app/igait-human-gait-detection

# Copy DeepSort pretrained weights from the repository
COPY igait-stages/igait-stage2-validity-check/igait-human-gait-detection/deepsort_parameters/ckpt.t7 \
     /app/igait-human-gait-detection/deep_sort/deep_sort/deep/checkpoint/ckpt.t7

# Pre-cache YOLOv5 model (downloads repo + weights into torch hub cache)
RUN cd /app/igait-human-gait-detection && \
    python3 -c "\
import torch; \
model = torch.hub.load('ultralytics/yolov5', 'yolov5l6', device='cpu'); \
print('YOLOv5 model cached successfully')"

# Pre-cache SlowFast model weights
RUN python3 -c "\
from pytorchvideo.models.hub import slowfast_r50_detection; \
model = slowfast_r50_detection(True); \
print('SlowFast model cached successfully')"

# Copy the compiled Rust worker binary
COPY --from=builder /app/igait-stages/igait-stage2-validity-check/target/release/igait-stage2-validity-check /usr/local/bin/worker

ENV RUST_LOG=info
CMD ["worker"]
