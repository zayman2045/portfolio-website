mod routes;
mod entities;

use sea_orm::*;
use entities::{prelude::*, *};

const DATABASE_URL: &str = "postgres://postgres:mysecretpassword@database:5432/postgres";

// Run the server
pub async fn run() -> Result<(), DbErr> {
    let app = routes::create_router().await;

    // Connect to the database
    let db = match Database::connect(DATABASE_URL).await {
        Ok(db) => db,
        Err(error) => {
            eprintln!("Error connecting to the database: {:?}", error);
            panic!();
        }
    };

    let new_user = users::ActiveModel {
        username: ActiveValue::Set("new_username".to_owned()),
        password: ActiveValue::Set("new_password".to_owned()),
        ..Default::default()
    };

    let result = Users::insert(new_user).exec(&db).await?;
    
    // Serve the app
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}