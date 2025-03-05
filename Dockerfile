FROM --platform=amd64 rust:1.85-bullseye

COPY ./.cargo /root/.cargo

RUN apt update && apt upgrade -y

RUN rustup target add wasm32-unknown-unknown
RUN cargo install wasm-bindgen-cli
RUN cargo install trunk
RUN cargo install wasm-pack

COPY ./src /root/src
COPY ./Cargo.toml /root/Cargo.toml
COPY ./Cargo.lock /root/Cargo.lock
COPY ./index.html /root/index.html
COPY ./build.py /root/build.py
