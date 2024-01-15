//! Entrypoint for the backend server.
use backend::run;

/// Runs the backend server.
#[tokio::main]
async fn main() {
    run().await.unwrap();
}