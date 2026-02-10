FROM rust:1.93-alpine AS builder

RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static pkgconfig

WORKDIR /app

# Copy Cargo manifests for dependency caching
COPY igait-lib/Cargo.toml ./igait-lib/Cargo.toml
COPY igait-stages/igait-stage5-cycle-detection/Cargo.toml ./igait-stages/igait-stage5-cycle-detection/Cargo.toml

# Create dummy source files to cache dependencies
RUN mkdir -p igait-lib/src && echo "pub fn dummy() {}" > igait-lib/src/lib.rs
RUN mkdir -p igait-stages/igait-stage5-cycle-detection/src && echo "fn main() {}" > igait-stages/igait-stage5-cycle-detection/src/main.rs

# Build dependencies only (this layer will be cached)
WORKDIR /app/igait-stages/igait-stage5-cycle-detection
RUN cargo build --release 2>/dev/null || true

# Now copy actual source files
WORKDIR /app
COPY igait-lib ./igait-lib
COPY igait-stages/igait-stage5-cycle-detection ./igait-stages/igait-stage5-cycle-detection

# Build with actual source
WORKDIR /app/igait-stages/igait-stage5-cycle-detection
# Clean the dummy build artifacts first
RUN cargo clean --release
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates \
    python3 \
    python3-pip \
    && pip3 install --no-cache-dir --break-system-packages \
        numpy \
        pandas \
        scipy \
        matplotlib \
    && rm -rf /var/lib/apt/lists/*

# Copy the Python gait cycle detection script
COPY igait-stages/igait-stage5-cycle-detection/igait-gait-cycle-detection/gait_analysis_mediapipe.py /app/gait_analysis_mediapipe.py

COPY --from=builder /app/igait-stages/igait-stage5-cycle-detection/target/release/igait-stage5-cycle-detection /usr/local/bin/worker
ENV RUST_LOG=info
CMD ["worker"]
