FROM rust:slim
WORKDIR /app

COPY ./ ./
RUN cargo build

CMD cargo run
