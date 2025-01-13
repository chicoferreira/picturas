# Stage 1: Planner
FROM lukemathwalker/cargo-chef:latest-rust-latest AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin projects-ms

FROM debian:bookworm-slim AS runtime
WORKDIR /app

COPY --from=builder /app/target/release/projects-ms /usr/local/bin/projects-ms

EXPOSE 8000

CMD ["/usr/local/bin/projects-ms"]
