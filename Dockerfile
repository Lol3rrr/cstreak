FROM rust:1.79 AS builder

COPY . .

WORKDIR webui

RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk

RUN trunk build --release
