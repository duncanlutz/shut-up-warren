FROM rust:buster as builder

RUN rustup update
RUN rustup install 1.75.0 && rustup default 1.75.0

# Install cmake, clang, and specifically openssl version 1.1
RUN apt-get update                  \
    && apt-get install -y cmake    \
    && apt-get install -y clang    \
    && apt-get install -y libssl-dev \
    && apt-get install -y pkg-config

COPY Cargo.toml Cargo.lock ./
RUN mkdir src                       \
    && touch src/lib.rs
RUN cargo build --release

RUN rm -rf src
COPY src ./src

# avoid linking problems with openssl
ENV OPENSSL_LIB_DIR=/usr/lib/x86_64-linux-gnu

RUN cargo build --release

FROM debian:buster

RUN apt update                      \
    && apt install -y libssl1.1     \
    && rm -rf /var/lib/apt/lists/*

RUN apt-get update                  \
    && apt-get install -y ca-certificates

RUN update-ca-certificates

COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=builder /src /usr/local/bin/src
COPY --from=builder /target/release/shut_up_warren /usr/local/bin/app

WORKDIR /usr/local/bin

CMD ["./app"]
