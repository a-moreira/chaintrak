FROM rust:1.61-slim-bullseye

WORKDIR /app
COPY ./ /app

RUN apt update && apt -y install build-essential pkg-config libasound2-dev libssl-dev
RUN cargo build --release
RUN chmod +x /app/target/release/chaintrak
ENTRYPOINT ["/app/target/release/chaintrak"]
