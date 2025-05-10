FROM docker.io/ubuntu:focal AS builder

RUN apt-get update && apt-get upgrade --yes && apt-get install --yes git wget clang make

# Rust
ADD --chmod=755 https://sh.rustup.rs rustup.sh
ENV CARGO_HOME=/usr/local/cargo
ENV PATH=/usr/local/cargo/bin:$PATH
RUN ./rustup.sh -y --no-modify-path --profile minimal --default-toolchain 1.86.0 && rustc --version

# Prepare working directory
WORKDIR /lila-gif
COPY theme ./theme
COPY src ./src
COPY Cargo.toml Cargo.lock ./

# Run tests
RUN ls -R && cargo test

# Build
RUN cargo build --release

# Final image
FROM docker.io/ubuntu:focal
RUN apt-get update && apt-get upgrade --yes
COPY --from=builder /lila-gif/target/release/lila-gif /usr/local/bin/
ENTRYPOINT ["/usr/local/bin/lila-gif"]
