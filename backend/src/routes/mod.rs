mod users;

use axum::{
    http::HeaderValue,
    routing::{get, post},
    Extension, Router,
};

use hyper::Method;
use sea_orm::DatabaseConnection;
use tower_http::cors::{CorsLayer, Any};
use users::{create_user, get_user};

// Create Axum router, define paths and assign handler functions
pub async fn create_router(database: DatabaseConnection) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        //.allow_origin("http://localhost:8080".parse::<HeaderValue>().unwrap());
        .allow_origin(Any);

    Router::new()
        .route("/users", post(create_user))
        .route("/users", get(get_user))
        .layer(Extension(database))
        .layer(cors)
}
