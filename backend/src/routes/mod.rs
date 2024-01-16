//! Handles the routing of the web service.

pub mod users;
pub mod missions;

use axum::{
    routing::post,
    Extension, Router,
};

use hyper::Method;
use sea_orm::DatabaseConnection;
use tower_http::cors::{CorsLayer, Any};

/// Builds the router.
pub async fn create_router(database: DatabaseConnection) -> Router {
    // Enable CORS, allowing GET and POST requests from any origin
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    // Define the routes and attaches layers
    Router::new()
        .route("/users", post(users::create_user))
        .route("/login", post(users::login_user))
        .route("/missions", post(missions::create_mission))
        .layer(Extension(database))
        .layer(cors)
}
