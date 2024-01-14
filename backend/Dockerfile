FROM rust:1.74
WORKDIR /code
COPY . /code
RUN cargo build --release
CMD cargo run --release
EXPOSE 3000