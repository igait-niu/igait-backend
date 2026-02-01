FROM rust:1.83-alpine AS builder

RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static pkgconfig

WORKDIR /app
COPY igait-lib ./igait-lib
COPY igait-stages/igait-stage7-finalize ./igait-stages/igait-stage7-finalize

WORKDIR /app/igait-stages/igait-stage7-finalize
RUN cargo build --release

FROM alpine:latest
RUN apk add --no-cache ca-certificates libgcc
COPY --from=builder /app/igait-stages/igait-stage7-finalize/target/release/igait-stage7-finalize /usr/local/bin/worker
ENV RUST_LOG=info
CMD ["worker"]
