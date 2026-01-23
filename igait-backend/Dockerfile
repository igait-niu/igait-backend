# Multi-stage build for igait-backend
FROM rust:1.93-slim AS builder

WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
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

# Build the backend
RUN cargo build --release -p igait-backend

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary
COPY --from=builder /app/target/release/igait-backend /usr/local/bin/igait-backend

# Set environment
ENV RUST_LOG=info
ENV PORT=3000

EXPOSE 3000

CMD ["igait-backend"]
