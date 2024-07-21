FROM rust:1.79-slim
WORKDIR /app

RUN apt-get update && apt-get install -y \
    git \
    curl \
    libssl-dev \
    pkg-config \
    build-essential \
 && rm -rf /var/lib/apt/lists/*

# add developing tools
RUN cargo install --version ^0.15.0 cargo-outdated

# cargo build setup
RUN rustup target add `uname --machine`-unknown-linux-musl

COPY ./ ./
RUN cargo build --target `uname --machine`-unknown-linux-musl

CMD cargo run
