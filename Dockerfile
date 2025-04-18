FROM rust:latest

WORKDIR /usr/src/aitu-web-app

COPY Cargo.toml Cargo.lock ./

COPY src ./src

RUN cargo build --release

RUN cargo test --release

CMD ["./target/release/aitu-web-app"]
