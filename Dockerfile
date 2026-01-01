# syntax=docker/dockerfile:1
# Based on https://depot.dev/docs/container-builds/optimal-dockerfiles/rust-dockerfile

FROM rust:1 AS build

RUN cargo install cargo-chef --locked

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo chef prepare --recipe-path recipe.json

RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    cargo chef cook --release --recipe-path recipe.json

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    cargo build --release --bin lila-gif

FROM debian:trixie-slim AS runtime

RUN groupadd -g 1001 lichess && \
    useradd -u 1001 -g lichess -m -d /home/lichess -s /bin/bash lichess

COPY --from=build --chown=lichess:lichess /app/target/release/lila-gif /usr/local/bin/lila-gif

USER lichess

ENTRYPOINT ["/usr/local/bin/lila-gif"]
