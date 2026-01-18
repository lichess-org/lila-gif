# syntax=docker/dockerfile:1
# Based on https://depot.dev/docs/container-builds/optimal-dockerfiles/rust-dockerfile

FROM docker.io/ubuntu:focal AS build

RUN apt-get update && apt-get upgrade --yes && apt-get install --yes git wget clang make

# Rust
ADD --chmod=755 https://sh.rustup.rs rustup.sh
ENV CARGO_HOME=/usr/local/cargo
ENV PATH=/usr/local/cargo/bin:$PATH
RUN ./rustup.sh -y --no-modify-path --profile minimal --default-toolchain 1.90.0 && rustc --version

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

FROM docker.io/ubuntu:focal AS runtime

RUN groupadd -g 1001 lichess && \
    useradd -u 1001 -g lichess -m -d /home/lichess -s /bin/bash lichess

COPY --from=build --chown=lichess:lichess /app/target/release/lila-gif /usr/local/bin/lila-gif

USER lichess

ENTRYPOINT ["/usr/local/bin/lila-gif"]
