FROM lukemathwalker/cargo-chef:0.1.50-rust-buster AS chef
WORKDIR /app

RUN apt-get update -y && \
  apt-get install -y --no-install-recommends \
    cmake \
    g++ \
    libsasl2-dev \
    libssl-dev \
    libudev-dev \
    pkg-config \
    protobuf-compiler \
  && \
  rm -rf /var/lib/apt/lists/*

FROM chef AS planner
COPY Cargo.* rust-toolchain.toml ./
COPY api api
COPY migration migration
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY Cargo.* rust-toolchain.toml ./
COPY api api
COPY migration migration

FROM builder AS builder-hub-credits
RUN cargo build --release --bin holaplex-hub-credits

FROM builder AS builder-migration
RUN cargo build --release --bin migration

FROM debian:bullseye-slim as base
WORKDIR /app
RUN apt-get update -y && \
  apt-get install -y --no-install-recommends \
    ca-certificates \
    libpq5 \
    libssl1.1 \
  && \
  rm -rf /var/lib/apt/lists/*

RUN mkdir -p bin

FROM base AS hub-credits
COPY --from=builder-hub-credits /app/target/release/holaplex-hub-credits bin
CMD ["bin/holaplex-hub-credits"]

FROM base AS migrator
COPY --from=builder-migration /app/target/release/migration bin/
CMD ["bin/migration"]