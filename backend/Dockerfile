# Development Stage
FROM rust:1.82 as dev
RUN cargo install cargo-watch
WORKDIR /code
COPY . /code
EXPOSE 3000
ENV API_BASE_URL=http://localhost:8080
CMD cargo-watch -x run

# Production Build Stage
FROM rust:1.82 as builder
WORKDIR /usr/src/backend
RUN cargo init
COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml
ENV API_BASE_URL=http://www.xaviergriffith.com
RUN cargo build --release

# Production Runtime Stage
FROM rust:1.82-slim as prod
COPY --from=builder /usr/src/backend/target/release/backend /usr/local/bin/backend
CMD ["backend"]
