FROM rust:slim
WORKDIR /app

RUN apt-get update \
  && apt-get upgrade -y \
  && apt-get install -y \
  git

COPY ./ ./
RUN cargo build

CMD cargo run
