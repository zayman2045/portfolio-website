# Build stage
FROM rust:1.74 as builder
WORKDIR /usr/src/app
RUN rustup target add wasm32-unknown-unknown
RUN cargo install wasm-bindgen-cli --vers 0.2.87
RUN cargo install wasm-server-runner
COPY . .
RUN cargo build --release --target wasm32-unknown-unknown

# Runtime stage
FROM ubuntu:21.10
WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/wasm32-unknown-unknown/release/frontend.wasm .
COPY --from=builder /usr/local/cargo/bin/wasm-server-runner /usr/local/bin/
CMD ["wasm-server-runner", "frontend.wasm"]