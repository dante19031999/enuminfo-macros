FROM rust:latest

RUN cargo install cargo-deny --locked
RUN cargo install cargo-audit --locked
RUN cargo install cargo-about --locked
