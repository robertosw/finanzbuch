# Using the same version as the Dockerfile for building to aarch64
FROM rust:1.71.0

# Install all required system tools
RUN apt update 
RUN apt upgrade -y

# Rust formatter
RUN rustup toolchain install nightly
RUN rustup component add rustfmt --toolchain nightly-x86_64-unknown-linux-gnu
# RUN rustup component add rustfmt

RUN rustup target add x86_64-unknown-linux-musl
