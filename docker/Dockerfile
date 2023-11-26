# Using the same version as the Dockerfile for building to aarch64
FROM rust:1.74.0

RUN apt update && apt upgrade -y        # Install all required system tools
RUN rustup toolchain install nightly && rustup component add rustfmt --toolchain nightly-x86_64-unknown-linux-gnu       # Rust formatter

# Tauri
RUN apt install -y libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \ 
    && cargo install tauri-cli
