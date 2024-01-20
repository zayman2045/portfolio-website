//! Entrypoint for the backend server.

use backend::run;
// use dotenv::dotenv; // Uncomment in development
use std::env;

/// Runs the backend server.
#[tokio::main]
async fn main() {
    // dotenv().ok(); // Uncomment in development
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
    run(database_url).await.unwrap();
}
