//! User specific routes.

use axum::http::StatusCode;
use axum::{Extension, Json};
use sea_orm::*;
use serde::{Deserialize, Serialize};

use crate::entities::prelude::*;
use crate::entities::users::{self, Model};

/// The request body for creating a new user or logging in.
#[derive(Deserialize, Serialize)]
pub struct UserRequest {
    pub username: String,
    pub password: String,
}

// The response body for creating a new user or logging in.
#[derive(Deserialize, Serialize)]
pub struct UserResponse {
    pub username: String,
    pub id: i32,
    pub token: String,
}

/// Creates a new user in the database.
pub async fn create_user(
    Extension(database): Extension<DatabaseConnection>,
    Json(user_request): Json<UserRequest>,
) -> Result<Json<UserResponse>, StatusCode> {
    // Create a new user model / row in the database
    let new_user = users::ActiveModel {
        username: ActiveValue::Set(user_request.username.clone()),
        password: ActiveValue::Set(hash_password(user_request.password)?),
        token: ActiveValue::Set(Some("Create Token".to_string())),
        ..Default::default() // Return status code if user already exists
    }
    .save(&database)
    .await
    .map_err(|_e| StatusCode::CONFLICT)?;

    Ok(Json(UserResponse {
        username: new_user.username.unwrap(),
        id: new_user.id.unwrap(),
        token: new_user.token.unwrap().unwrap(),
    }))
}

/// Logs the user in.
pub async fn login_user(
    Extension(database): Extension<DatabaseConnection>,
    Json(user_request): Json<UserRequest>,
) -> Result<Json<UserResponse>, StatusCode> {
    // Query the database for the username
    let database_user = Users::find()
        .filter(users::Column::Username.eq(user_request.username.clone()))
        .one(&database)
        .await
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Username found in database
    if let Some(database_user) = database_user {
        if !verify_password(user_request.password, &database_user.password)? {
            return Err(StatusCode::UNAUTHORIZED);
        }

        // Update the token
        let new_token = "Create Token".to_string();
        let mut user = database_user.into_active_model();
        user.token = Set(Some(new_token));

        // Save the user to the database
        let saved_user = user
            .save(&database)
            .await
            .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json(UserResponse {
            username: saved_user.username.unwrap(),
            id: saved_user.id.unwrap(),
            token: saved_user.token.unwrap().unwrap(),
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

/// Logs the user out.
pub async fn logout_user(
    Extension(database): Extension<DatabaseConnection>,
    Extension(user): Extension<Model>,
) -> Result<(), StatusCode> {
    let mut user = user.into_active_model();
    user.token = Set(None);
    user.save(&database).await.map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(())
}

/// Hashes the password before storage in the database.
fn hash_password(password: String) -> Result<String, StatusCode> {
    bcrypt::hash(password, 10).map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)
}

// Verifies the password in the request matches that of the database.
fn verify_password(password: String, hash: &str) -> Result<bool, StatusCode> {
    bcrypt::verify(password, hash).map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)
}
