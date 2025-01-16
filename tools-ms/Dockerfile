FROM lukemathwalker/cargo-chef:latest-rust-latest AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

RUN apt-get update && apt-get install -y \
    libleptonica-dev \
    libclang-dev \
    libtesseract-dev \
    && rm -rf /var/lib/apt/lists/*

COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin tools-ms

FROM debian:bookworm-slim AS runtime
WORKDIR /app

COPY --from=builder /app/target/release/tools-ms /usr/local/bin/tools-ms

CMD ["/usr/local/bin/tools-ms"]
