FROM rust:1.93-slim AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy Cargo manifests for dependency caching
COPY igait-lib/Cargo.toml ./igait-lib/Cargo.toml
COPY igait-stages/igait-stage4-pose-estimation/Cargo.toml ./igait-stages/igait-stage4-pose-estimation/Cargo.toml

# Create dummy source files to cache dependencies
RUN mkdir -p igait-lib/src && echo "pub fn dummy() {}" > igait-lib/src/lib.rs
RUN mkdir -p igait-stages/igait-stage4-pose-estimation/src && echo "fn main() {}" > igait-stages/igait-stage4-pose-estimation/src/main.rs

# Build dependencies only (this layer will be cached)
WORKDIR /app/igait-stages/igait-stage4-pose-estimation
RUN cargo build --release 2>/dev/null || true

# Now copy actual source files
WORKDIR /app
COPY igait-lib ./igait-lib
COPY igait-stages/igait-stage4-pose-estimation ./igait-stages/igait-stage4-pose-estimation

# Build with actual source
WORKDIR /app/igait-stages/igait-stage4-pose-estimation
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates libssl3 python3 python3-pip python3-opencv \
    && pip3 install --no-cache-dir --break-system-packages mediapipe numpy \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/igait-stages/igait-stage4-pose-estimation/target/release/igait-stage4-pose-estimation /usr/local/bin/worker
ENV RUST_LOG=info
CMD ["worker"]
