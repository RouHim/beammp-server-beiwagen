FROM rust:latest AS chef
# We only pay the installation cost once,
# it will be cached from the second build onwards
RUN cargo install cargo-chef
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release
RUN strip /app/target/release/beammp-server-beiwagen

# We do not need the Rust toolchain to run the binary!
FROM scratch AS runtime
# Mods directory to synchronize
ENV BEAMMP_CLIENT_MODS_DIR="/mods"
# A list of mod ids that should be keep track of
ENV BEAMMP_MODS=""
# How to handle outdated entries. May be empty or `delete` or `skip`
ENV OUTDATED=""
# How to handle unsupported entries. May be empty or `delete` or `skip`
ENV UNSUPPORTED=""

WORKDIR app
COPY --from=builder /app/target/release/beammp-server-beiwagen /beiwagen
ENTRYPOINT ["/beiwagen"]