FROM rust:1-bookworm

WORKDIR /app

COPY src/ ./src/
COPY Cargo.toml Cargo.lock .

RUN cargo build --release

EXPOSE 3000

CMD ./target/release/frottage-backend
