FROM rust:latest AS builder

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /usr/src/app

RUN apt-get update && apt-get install -y git && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/git-remote-pull /usr/local/bin/git-remote-pull

EXPOSE 9999

ENTRYPOINT ["/usr/local/bin/git-remote-pull"]
