FROM rust:1 AS build-stage
ADD Cargo.toml /sutekh/Cargo.toml
ADD src /sutekh/src
WORKDIR /sutekh
RUN cargo build --release
FROM debian:stretch-slim
RUN apt-get update
RUN apt-get install -y apt-transport-https dirmngr
RUN echo 'deb https://apt.dockerproject.org/repo debian-stretch main' >> /etc/apt/sources.list
RUN apt-key adv --keyserver-options http-proxy=$http_proxy --keyserver hkp://p80.pool.sks-keyservers.net:80 --recv-keys 58118E89F3A912897C070ADBF76221572C52609D
RUN apt-get update
RUN apt-get install -y docker-engine
COPY --from=build-stage /sutekh/target/release/sutekh /usr/local/bin/sutekh
CMD /usr/local/bin/sutekh
