//! Entrypoint for the backend server.

use backend::run;
use dotenv::dotenv;
use std::env;


/// Runs the backend server.
#[tokio::main]
async fn main() {
    dotenv().ok();
    run().await.unwrap();
}