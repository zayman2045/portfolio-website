//! Handles the routing of the web service.

pub mod guards;
pub mod missions;
pub mod users;

use axum::{
    http::HeaderValue,
    middleware,
    routing::{delete, get, post},
    Extension, Router,
};

use hyper::{
    header::{AUTHORIZATION, CONTENT_TYPE},
    Method,
};
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

/// Builds the router.
pub async fn create_router(database: DatabaseConnection) -> Router {
    // Wrap the database connection in an Arc to share it between threads
    let database = Arc::new(database);

    // Import the API base URL from the environment
    let env_api_base_url = match std::env::var("API_BASE_URL") {
        Ok(val) => val.parse::<HeaderValue>().expect("Parse"),
        Err(_) => {
            eprintln!("API_BASE_URL not set, defaulting to http://localhost:8080");
            "http://localhost:8080"
                .parse::<HeaderValue>()
                .expect("Parse")
        }
    };

    // Define the allowed origins
    let origins = [
        env_api_base_url,
        "https://xaviergriffith.com"
            .parse()
            .expect("Failed to parse HeaderValue"),
        "https://www.xaviergriffith.com"
            .parse()
            .expect("Failed to parse HeaderValue"),
    ];

    // Enable CORS, allowing GET, POST and DELETE requests
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_origin(origins)
        .allow_headers(vec![CONTENT_TYPE, AUTHORIZATION]);

    // Define the routes, assign handlers, and attaches layers
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
