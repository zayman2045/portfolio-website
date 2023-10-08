mod hello_server;
mod users;

use axum::{routing::{get, post}, Router, Extension};
use hello_server::hello_server;

use sea_orm::DatabaseConnection;
use users::{create_user, get_user};

// Create Axum router, define paths and assign handler functions
pub async fn create_router(database: DatabaseConnection) -> Router {
    Router::new()
    .route("/hello-server", get(hello_server))
    .route("/users", post(create_user))
    .route("/users", get(get_user))
    .layer(Extension(database))
}
