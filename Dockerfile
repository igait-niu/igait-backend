# syntax=docker/dockerfile:1
FROM rust:1.75-slim as build
WORKDIR /
COPY . .
RUN apt-get update
RUN apt-get install -y pkg-config curl
RUN apt-get install -y libssl-dev openssl
RUN ["cargo", "build"]

FROM python:3.12-slim 
COPY --from=build /target/debug/igait-backend /igait-backend
RUN pip install opencv-python
RUN apt update
RUN apt install -y openssh-client
VOLUME /data
VOLUME /root/.ssh
CMD ["/igait-backend"]
EXPOSE 3000
