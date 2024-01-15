//! Core functionality for the server-side web service.
//!
//! Connects to the database and serves the API.

pub mod entities;
pub mod routes;

use sea_orm::*;

const DATABASE_URL: &str = "postgres://postgres:mysecretpassword@database:5432/postgres";

/// Runs the server and connects to the database.
pub async fn run() -> Result<(), DbErr> {
    // Connect to the database
    let db = match Database::connect(DATABASE_URL).await {
        Ok(db) => db,
        Err(error) => {
            eprintln!("Error connecting to the database: {:?}", error);
            panic!();
        }
    };

    let app = routes::create_router(db).await;

    // Serve the app
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
