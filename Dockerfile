# syntax=docker/dockerfile:1
FROM rust:1.75-slim as build
WORKDIR /
COPY . .
RUN apt-get update
RUN apt-get install -y pkg-config curl
RUN apt-get install -y libssl-dev openssl
RUN ["cargo", "build"]

FROM ubuntu:22.04
COPY --from=build /target/debug/igait-backend /igait-backend
ARG DEBIAN_FRONTEND=noninteractive
RUN apt update
RUN apt install -y openssh-client openssl libssl-dev pkg-config
VOLUME /data
VOLUME /root/.ssh
CMD ["/igait-backend"]
EXPOSE 3000
