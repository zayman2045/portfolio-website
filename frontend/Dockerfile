FROM rust:1.74
WORKDIR /usr/src/app
RUN rustup target add wasm32-unknown-unknown
RUN cargo install wasm-bindgen-cli --vers 0.2.89
RUN cargo install wasm-server-runner
COPY . .
RUN cargo build --release --target wasm32-unknown-unknown
CMD ["cargo", "run", "--release", "--target", "wasm32-unknown-unknown"]
