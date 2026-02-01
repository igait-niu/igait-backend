FROM rust:1.93-slim AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY igait-lib ./igait-lib
COPY igait-stages/igait-stage6-prediction ./igait-stages/igait-stage6-prediction

WORKDIR /app/igait-stages/igait-stage6-prediction
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates libssl3 python3 python3-pip \
    && pip3 install --no-cache-dir --break-system-packages tensorflow numpy scikit-learn \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/igait-stages/igait-stage6-prediction/target/release/igait-stage6-prediction /usr/local/bin/worker
# COPY models/ /app/models/  # Add trained ML model when ready
ENV RUST_LOG=info
CMD ["worker"]
