# Build stage
FROM rust:1.74 as builder
WORKDIR /usr/src/backend
RUN cargo init
COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release

# Runtime stage
FROM rust:1.74-slim
COPY --from=builder /usr/src/backend/target/release/backend /usr/local/bin/backend
EXPOSE 8080
CMD ["backend"]