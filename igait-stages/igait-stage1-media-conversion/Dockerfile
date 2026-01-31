FROM rust:1.93-alpine AS builder

# Install build dependencies
RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    openssl-libs-static \
    pkgconfig

WORKDIR /app

# Copy lib and stage files
COPY igait-lib ./igait-lib
COPY igait-stages/igait-stage1-media-conversion ./igait-stages/igait-stage1-media-conversion

# Build the service
WORKDIR /app/igait-stages/igait-stage1-media-conversion
RUN cargo build --release

# Runtime image
FROM alpine:latest

# Install FFmpeg for video conversion
RUN apk add --no-cache \
    ffmpeg \
    ca-certificates \
    libgcc

# Copy binary from builder
COPY --from=builder /app/igait-stages/igait-stage1-media-conversion/target/release/igait-stage1-media-conversion /usr/local/bin/stage-service

# Set environment variables
ENV RUST_LOG=info
ENV PORT=8080

EXPOSE 8080

CMD ["stage-service"]
