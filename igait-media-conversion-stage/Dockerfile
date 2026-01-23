FROM rust:1.93-alpine AS builder

# Install build dependencies
RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    openssl-libs-static \
    pkgconfig

WORKDIR /app

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

# Build the service
RUN cargo build --release -p igait-media-conversion-stage

# Runtime image
FROM alpine:latest

# Install FFmpeg for video conversion
RUN apk add --no-cache \
    ffmpeg \
    ca-certificates \
    libgcc

# Copy binary from builder
COPY --from=builder /app/target/release/igait-media-conversion-stage /usr/local/bin/stage-service

# Set environment variables
ENV RUST_LOG=info
ENV PORT=8080

EXPOSE 8080

CMD ["stage-service"]
