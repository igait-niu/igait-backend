ARG RUSTC_VERSION=1.75
ARG PYTHON_VERSION=3.12.1
FROM rust:${RUSTC_VERSION}-slim as base

WORKDIR /

COPY . .

VOLUME /app/data

RUN apt-get update
RUN apt-get install -y pkg-config
RUN apt-get install -y libssl-dev openssl

CMD ["cargo", "run"]

EXPOSE 3000