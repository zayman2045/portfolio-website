# Development Stage
FROM rust:1.82 as dev
ENV API_BASE_URL=http://127.0.0.1:3000
RUN rustup target add wasm32-unknown-unknown && cargo install trunk
WORKDIR /code
COPY . /code
EXPOSE 8080
CMD trunk serve --address 0.0.0.0

# Production Build Stage
FROM rust:1.82 as builder
ENV API_BASE_URL=http://xog-backend-v2.us-east-2.elasticbeanstalk.com
WORKDIR /usr/src/app
RUN rustup target add wasm32-unknown-unknown && cargo install trunk
COPY . .
RUN trunk build --release

# Production Runtime Stage
FROM nginx:alpine as prod
WORKDIR /usr/share/nginx/html
COPY --from=builder /usr/src/app/dist .
COPY nginx.conf /etc/nginx/nginx.conf
CMD ["nginx", "-g", "daemon off;"]