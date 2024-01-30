//! Handles the routing of the web service.

pub mod guards;
pub mod missions;
pub mod users;

use axum::{
    middleware,
    routing::{delete, get, post},
    Extension, Router,
};

use hyper::{
    header::{AUTHORIZATION, CONTENT_TYPE},
    Method,
};
use sea_orm::DatabaseConnection;
use tower_http::cors::{Any, CorsLayer};

/// Builds the router.
pub async fn create_router(database: DatabaseConnection) -> Router {
    // Enable CORS, allowing GET, POST and DELETE requests
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_origin(Any)
        .allow_headers(vec![CONTENT_TYPE, AUTHORIZATION]);

    // Define the routes and attaches layers
    Router::new()
        .route("/users/logout", post(users::logout_user))
        .route("/missions", post(missions::create_mission))
        .route("/users/:user_id", get(missions::list_missions))
        .route("/missions/:mission_id", get(missions::get_mission))
        .route("/missions/:mission_id", post(missions::update_mission))
        .route("/missions/:mission_id", delete(missions::delete_mission))
        .route_layer(middleware::from_fn(guards::token_guard))
        .route("/users", post(users::create_user))
        .route("/login", post(users::login_user))
        .layer(Extension(database))
        .layer(cors)
}
