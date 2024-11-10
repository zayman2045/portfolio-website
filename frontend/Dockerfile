# # Development Stage (Only Uncomment in Development)
FROM rust:1.74
RUN rustup target add wasm32-unknown-unknown && cargo install trunk
WORKDIR /code
COPY . /code
EXPOSE 8080
CMD trunk serve --address 127.0.0.1

# Production Build Stage (Only Uncomment in Production)
# FROM rust:1.74 as builder
# WORKDIR /usr/src/app
# RUN rustup target add wasm32-unknown-unknown && cargo install trunk
# COPY . .
# RUN trunk build --release

# # Production Runtime Stage (Only Uncomment in Production)
# FROM nginx:alpine
# WORKDIR /usr/share/nginx/html
# COPY --from=builder /usr/src/app/dist .
# COPY config.json .  
# COPY nginx.conf /etc/nginx/nginx.conf
# CMD ["nginx", "-g", "daemon off;"]