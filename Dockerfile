FROM rust:1.56 as builder
RUN apt-get update && apt install -y cmake

WORKDIR /usr/src/matrix-lightning-tip-bot
COPY src ./src
COPY Cargo.toml .
COPY diesel.toml .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y ca-certificates libssl-dev sqlite3 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/matrix-lightning-tip-bot /usr/local/bin/matrix-lightning-tip-bot
