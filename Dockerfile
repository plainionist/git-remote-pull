FROM rust:latest AS builder

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

FROM alpine:latest

WORKDIR /usr/src/app

RUN apk add --no-cache git

COPY --from=builder /usr/src/app/target/release/git_pull_web /usr/local/bin/git_pull_web

EXPOSE 9999

ENTRYPOINT ["/usr/local/bin/git_pull_web"]
