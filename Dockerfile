FROM rust:1.79 AS builder

COPY . .

WORKDIR webui

RUN rustup toolchain install nightly
RUN rustup default nightly
RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk

RUN trunk build --release
