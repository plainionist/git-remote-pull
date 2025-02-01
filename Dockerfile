FROM rust:latest AS builder

RUN apt-get update && apt-get install -y musl-tools
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM alpine:latest

WORKDIR /usr/src/app

RUN apk add --no-cache git

COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/git_pull_web /usr/local/bin/git_pull_web

EXPOSE 9999

ENTRYPOINT ["/usr/local/bin/git_pull_web"]
