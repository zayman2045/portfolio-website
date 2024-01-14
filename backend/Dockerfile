# FROM rust:latest
# RUN cargo install cargo-watch
# WORKDIR /code
# COPY . /code
# EXPOSE 3000
# CMD cargo-watch -x run

FROM rust:1.74
WORKDIR /code
COPY . /code
RUN cargo build --release
CMD cargo run --release
EXPOSE 3000