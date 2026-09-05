FROM rust:latest

RUN cargo install cargo-deny cargo-audit cargo-about --locked