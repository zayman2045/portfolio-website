//! Entrypoint for the backend server.

use backend::run;
use std::env;

/// Runs the backend server.
#[tokio::main]
async fn main() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in environment");
    run(database_url).await.unwrap();
}
