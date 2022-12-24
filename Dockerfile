FROM rust:bullseye

RUN apt-get update -y && apt-get install npm -y
RUN rustup toolchain install stable && rustup target add wasm32-unknown-unknown
RUN bash -cl "wget -qO- https://github.com/thedodd/trunk/releases/download/v0.16.0/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf- && mv ./trunk /usr/bin/"

COPY . /app
WORKDIR /app
RUN trunk build

# Static output located at /app/dist