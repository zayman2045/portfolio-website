use axum::{response::IntoResponse, Router, routing::get, http::Method};
use tower_http::cors::{CorsLayer, Any};

#[tokio::main]
async fn main() {
    let cors_layer = CorsLayer::new()
        .allow_methods(Method::GET)
        .allow_origin(Any);

    let app = Router::new()
        .route("/hello-server", get(hello))
        .layer(cors_layer);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello() -> impl IntoResponse {
    "Hello from the Backend!"
}
