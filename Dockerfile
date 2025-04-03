FROM rust:1.86-bookworm as builder
WORKDIR /usr/src/app

COPY . .
COPY --from=d3fk/tailwindcss:stable /tailwindcss /usr/local/bin/tailwindcss
RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y procps ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/docker-registry-cleaner /usr/local/bin/docker-registry-cleaner

ENTRYPOINT ["/usr/local/bin/docker-registry-cleaner"]
