# Multi-stage build for Stage 6: Prediction
FROM rust:1.75-slim as builder

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
COPY igait-prediction-stage ./igait-prediction-stage

# Build the stage service
RUN cargo build --release -p igait-prediction-stage

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    python3 \
    python3-pip \
    && rm -rf /var/lib/apt/lists/*

# Install Python ML dependencies
RUN pip3 install --no-cache-dir tensorflow numpy scikit-learn

# Copy the binary
COPY --from=builder /app/target/release/igait-prediction-stage /usr/local/bin/stage-service

# Copy ML model (you'll need to add your trained model here)
# COPY models/ /app/models/

ENV RUST_LOG=info
ENV PORT=8080

EXPOSE 8080

CMD ["stage-service"]
