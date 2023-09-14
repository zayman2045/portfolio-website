mod hello_server;

use axum::{routing::get, Router};
use hello_server::hello_server;

// Create Axum router, define paths and assign handler functions
pub async fn create_router() -> Router {
    Router::new().route("/hello-server", get(hello_server))
}
