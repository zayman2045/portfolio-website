//! Route guarding middleware.

use std::sync::Arc;

use crate::{
    entities::{prelude::*, users},
    utils::jwt::validate_jwt,
};
use axum::{
    headers::{authorization::Bearer, Authorization, HeaderMapExt},
    http::Request,
    middleware::Next,
    response::Response,
};
use hyper::StatusCode;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

/// Intercepts requests to validate the JWT token. If the token is valid, the user is looked up in the database and inserted into the request extensions.
pub async fn token_guard<T>(
    mut request: Request<T>,
    next: Next<T>,
) -> Result<Response, StatusCode> {
    // Get the token from the request header
    let token = match request.headers().typed_get::<Authorization<Bearer>>() {
        Some(auth) => {
            println!("Token found: {:?}", auth.token());
            auth.token().to_owned()
        }
        None => {
            eprintln!("Failed to get token from request.");
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    // Validate the token
    validate_jwt(&token)?;

    // Get the database connection
    let database = match request.extensions().get::<Arc<DatabaseConnection>>() {
        Some(db) => {
            println!("Database connection found.");
            db
        }
        None => {
            eprintln!("Failed to get database connection from request.");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let x = &**database;
    println!("Database: {:?}", x);

    // Look the user up in the database using the token
    let user = match Users::find()
        .filter(users::Column::Token.eq(Some(token.clone())))
        .one(&**database)
        .await
    {
        Ok(Some(user)) => {
            println!("User found: {:?}", user);
            user
        }
        Ok(None) => {
            eprintln!("User not found.");
            return Err(StatusCode::UNAUTHORIZED);
        }
        Err(e) => {
            eprintln!("Failed to find user in database: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    println!("User found: {:?}", user);

    // Insert the user into the extensions
    request.extensions_mut().insert(user);

    Ok(next.run(request).await)
}
