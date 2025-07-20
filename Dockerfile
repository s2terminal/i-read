FROM rust:1.88-slim
WORKDIR /app

RUN apt-get update && apt-get install -y \
    git \
    curl \
    libssl-dev \
    pkg-config \
    build-essential \
 && rm -rf /var/lib/apt/lists/*

RUN git config --global --add safe.directory /app

# add developing tools
# RUN cargo install --version ^0.15.0 cargo-outdated
RUN rustup component add rustfmt clippy

# cargo build setup
RUN rustup target add `uname --machine`-unknown-linux-musl

COPY ./ ./
RUN cargo build --target `uname --machine`-unknown-linux-musl

CMD cargo run
