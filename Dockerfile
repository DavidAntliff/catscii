# syntax = docker/dockerfile:1.4
FROM ubuntu:20.04

RUN set -eux; \
    apt update; \
    apt install -y --no-install-recommends \
        curl ca-certificates gcc libc6-dev pkg-config libssl-dev \
        ;

RUN --mount=type=cache,target=/root/.rustup \
    set -eux; \
    curl --location --fail \
        "https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init" \
        --output rustup-init; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --default-toolchain stable; \
    rm rustup-init;

ENV PATH=${PATH}:/root/.cargo/bin
RUN set -eux; \
		rustup --version;

WORKDIR /app
COPY src src
COPY Cargo.toml Cargo.lock ./

RUN --mount=type=cache,target=/root/.rustup \
    --mount=type=cache,target=/root/.cargo/registry \
    --mount=type=cache,target=/root/.cargo/git \
    --mount=type=cache,target=/app/target \
		set -eux; \
        cargo build --release; \
        cp target/release/catscii .

CMD ["/app/catscii"]
