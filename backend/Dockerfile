# # Development Stage (Only Uncomment in Development)
FROM rust:1.75
RUN cargo install cargo-watch
WORKDIR /code
COPY . /code
EXPOSE 3000
CMD cargo-watch -x run

# Production Build Stage (Only Uncomment in Production)
# FROM rust:1.74 as builder
# WORKDIR /usr/src/backend
# RUN cargo init
# COPY ./src ./src
# COPY ./Cargo.toml ./Cargo.toml
# RUN cargo build --release

# # Production Runtime Stage (Only Uncomment in Production)
# FROM rust:1.74-slim
# COPY --from=builder /usr/src/backend/target/release/backend /usr/local/bin/backend
# CMD ["backend"]
