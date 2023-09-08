use axum::{response::IntoResponse, Router, routing::get};

#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/hello-server", get(hello));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello() -> impl IntoResponse {
    "Hello from the Backend!"
}
