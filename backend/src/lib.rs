mod routes;

use sea_orm::{Database, DbErr};

const DATABASE_URL: &str = "postgres://postgres:mysecretpassword@database:5432/postgres";

// Run the server
pub async fn run() -> Result<(), DbErr> {
    let app = routes::create_router().await;

    // Connect to the database
    let _db = match Database::connect(DATABASE_URL).await {
        Ok(db) => db,
        Err(error) => {
            eprintln!("Error connecting to the database: {:?}", error);
            panic!();
        }
    };
    
    // Serve the app
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}