use axum::{response::IntoResponse, routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/hello-server", get(hello))
    .route("/", get(home_page));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello() -> impl IntoResponse {
    "Hello from the Backend!"
}

async fn home_page() -> impl IntoResponse {
    "Home Page!"
}
