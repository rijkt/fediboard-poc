FROM docker.io/lukemathwalker/cargo-chef:latest-rust-1.90-slim-trixie AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin fediboard-poc

FROM debian:trixie-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/fediboard-poc /usr/local/bin
ENTRYPOINT ["/usr/local/bin/fediboard-poc"]