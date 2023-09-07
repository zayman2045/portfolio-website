// use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use axum::body::{boxed, Body};
use axum::http::{Response, StatusCode};
use axum::response::IntoResponse;
use axum::{routing::get, Router};
use tower::ServiceExt;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello))
        // Serve the static frontend files from ./dist as a fallback

        // .fallback_service(get(|req| async move {
        //     match ServeDir::new("./dist").oneshot(req).await {
        //         Ok(res) => res.map(boxed),
        //         Err(err) => Response::builder()
        //             .status(StatusCode::INTERNAL_SERVER_ERROR)
        //             .body(boxed(Body::from(format!("error: {err}"))))
        //             .expect("error response"),
        //     }
        // }))
        ;

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello() -> impl IntoResponse {
    "Hello from the Backend"
}
