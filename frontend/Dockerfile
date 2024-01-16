# Development Stage
FROM rust:1.74
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
WORKDIR /code
COPY . /code
EXPOSE 8080
CMD trunk serve --address 0.0.0.0

# # Production Build Stage
# FROM rust:1.74 as builder
# WORKDIR /usr/src/app
# RUN rustup target add wasm32-unknown-unknown && cargo install trunk
# COPY . .
# RUN trunk build --release

# # Production Runtime Stage
# FROM nginx:alpine
# WORKDIR /usr/share/nginx/html
# COPY --from=builder /usr/src/app/dist .
# COPY nginx.conf /etc/nginx/nginx.conf
# CMD ["nginx", "-g", "daemon off;"]