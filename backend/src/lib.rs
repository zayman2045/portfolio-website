//! Core functionality for the server-side web service.
//!
//! Connects to the database and serves the API.

pub mod entities;
pub mod routes;
pub mod utils;

use sea_orm::*;

/// Runs the server and connects to the database.
pub async fn run(database_url: String) -> Result<(), DbErr> {
    // Connect to the database
    let db = match Database::connect(database_url).await {
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
