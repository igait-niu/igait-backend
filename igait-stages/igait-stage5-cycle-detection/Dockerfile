FROM rust:1.93-alpine AS builder

RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static pkgconfig

WORKDIR /app
COPY igait-lib ./igait-lib
COPY igait-stages/igait-stage5-cycle-detection ./igait-stages/igait-stage5-cycle-detection

WORKDIR /app/igait-stages/igait-stage5-cycle-detection
RUN cargo build --release

FROM alpine:latest
RUN apk add --no-cache ca-certificates libgcc
COPY --from=builder /app/igait-stages/igait-stage5-cycle-detection/target/release/igait-stage5-cycle-detection /usr/local/bin/worker
ENV RUST_LOG=info
CMD ["worker"]
