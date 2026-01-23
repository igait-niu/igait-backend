# Multi-stage build for Stage 3: Reframing
FROM rust:1.75-alpine as builder

WORKDIR /app

# Install build dependencies
RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    pkgconfig

# Copy workspace files
COPY Cargo.toml ./
COPY igait-lib ./igait-lib
COPY igait-reframing-stage ./igait-reframing-stage

# Build the stage service
RUN cargo build --release -p igait-reframing-stage

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
