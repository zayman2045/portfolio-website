FROM rust:latest
RUN cargo install cargo-watch
WORKDIR /code
COPY . /code
EXPOSE 3000
CMD cargo-watch -x run