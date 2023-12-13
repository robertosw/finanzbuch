# Using the same version as the Dockerfile for building to aarch64
FROM rust:1.74.0

RUN apt update && apt upgrade -y

# Rust formatter
RUN rustup toolchain install nightly && \
    rustup component add rustfmt --toolchain nightly-x86_64-unknown-linux-gnu

# Tauri
RUN apt install -y \
    libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \ 
    && cargo install tauri-cli

# Flatpak
RUN apt install -y \
	kmod \
    flatpak \
    flatpak-builder && \
    flatpak remote-add --if-not-exists flathub https://dl.flathub.org/repo/flathub.flatpakrepo  && \
    flatpak install -y flathub org.freedesktop.Platform//23.08 org.freedesktop.Sdk//23.08

# kmod is just used to have modprobe
