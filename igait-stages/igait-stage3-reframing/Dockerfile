# Multi-stage build for Stage 3: Reframing
FROM rust:1.93-alpine as builder

WORKDIR /app

# Install build dependencies
RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    openssl-libs-static \
    pkgconfig

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
RUN cargo build --release -p igait-stage3-reframing

# Runtime stage
FROM alpine:latest

# Install runtime dependencies (FFmpeg for video manipulation)
RUN apk add --no-cache \
    ffmpeg \
    ca-certificates \
    libgcc

# Copy the binary
COPY --from=builder /app/target/release/igait-reframing-stage /usr/local/bin/stage-service

ENV RUST_LOG=info
ENV PORT=8080

EXPOSE 8080

CMD ["stage-service"]
