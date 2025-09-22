FROM rust:1.90 AS builder
WORKDIR /usr/src/fediboard-poc
COPY . .
RUN cargo install --path .

FROM debian:trixie-slim
RUN apt update && apt upgrade -y
COPY --from=builder /usr/local/cargo/bin/fediboard-poc /usr/local/bin/fediboard-poc
CMD ["fediboard-poc"]
