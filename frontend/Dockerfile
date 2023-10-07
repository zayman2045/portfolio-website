FROM rust:latest
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
WORKDIR /code
COPY . /code
EXPOSE 8080
CMD trunk serve --address 0.0.0.0
