//! Route guarding middleware.

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

/// Checks if the user is logged in and inserts the user into the request extensions.
pub async fn token_guard<T>(
    mut request: Request<T>,
    next: Next<T>,
) -> Result<Response, StatusCode> {
    // Get the token from the request header
    let token = request
        .headers()
        .typed_get::<Authorization<Bearer>>()
        .ok_or({
            eprintln!("Failed to get token from request");
            StatusCode::BAD_REQUEST
        })?
        .token()
        .to_owned();

    // Validate the token
    validate_jwt(&token)?;

    // Get the database connection
    let database = request.extensions().get::<DatabaseConnection>().ok_or({
        eprintln!("Failed to get database connection from request");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Look the user up in the database using the token
    let Some(user) = Users::find()
        .filter(users::Column::Token.eq(Some(token.clone())))
        .one(database)
        .await
        .map_err(|_e| {
            eprintln!("Failed to find user in database");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
    else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    // Insert the user into the extensions
    request.extensions_mut().insert(user);

    Ok(next.run(request).await)
}
