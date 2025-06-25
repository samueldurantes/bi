# Ref: https://loige.co/building_x86_rust-containers-from-mac-silicon/

FROM rust:1.87-slim-bullseye AS build

ENV SQLX_OFFLINE=true

RUN apt update && apt install -y --no-install-recommends clang llvm perl
RUN update-ca-certificates
RUN rustup target add x86_64-unknown-linux-musl

COPY . /app/bi

ENV CC_x86_64_unknown_linux_musl=clang
ENV RUST_BACKTRACE=full

RUN \
  --mount=type=cache,target=/app/bi/target \
  --mount=type=cache,target=/usr/local/cargo/registry \
  cd /app/bi && \
  cargo build --target x86_64-unknown-linux-musl --release && \
  cp target/x86_64-unknown-linux-musl/release/bi /app/bi/server

FROM alpine:3.17.3

RUN mkdir -p /app

COPY --from=build /app/bi/server /app/bi

WORKDIR /app

CMD ["./bi"]
