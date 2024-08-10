# Leveraging the pre-built Docker images with
# cargo-chef and the Rust toolchain
# must match the rust version you are developing with
FROM lukemathwalker/cargo-chef:latest-rust-1.76 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --recipe-path recipe.json

COPY . .
RUN cargo build --release

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime 
COPY --from=builder /app/target/release/drivebox /usr/local/bin
ENTRYPOINT ["/usr/local/bin/drivebox"]
