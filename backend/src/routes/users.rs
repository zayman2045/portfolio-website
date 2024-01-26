//! Handles user specific routes.

use axum::http::StatusCode;
use axum::{Extension, Json};
use sea_orm::*;
use serde::{Deserialize, Serialize};

use crate::entities::prelude::*;
use crate::entities::users;

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
    // Create a new user model
    let new_user = users::ActiveModel {
        username: ActiveValue::Set(user_request.username.clone()),
        password: ActiveValue::Set(hash_password(user_request.password)?),
        ..Default::default()
    };

    // Check if the user already exists
    match Users::find()
        .filter(users::Column::Username.eq(user_request.username.clone()))
        .one(&database)
        .await
    {
        Ok(user) => {
            // Return a conflict status code if the user is found
            if user.is_some() {
                return Err(StatusCode::CONFLICT);
            } else {
                // Insert the new user into the database
                match Users::insert(new_user).exec(&database).await {
                    Ok(insert_result) => {
                        return Ok(Json(UserResponse {
                            username: user_request.username,
                            id: insert_result.last_insert_id,
                            token: String::from("Hello World!"),
                        }))
                    }
                    Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
                };
            }
        }
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
}

/// Logs in a user.
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

        let new_token = "HelloWorld".to_string();
        let mut user = database_user.into_active_model();

        // user.token = Set(Some(new_token));

        // let saved_user = user.save(&database).await.map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR);

        // Ok(Json(UserResponse {
        //     username: saved_user.username.unwrap(),
        //     id: saved_user.id.unwrap(),
        //     token: saved_user.token.unwrap().unwrap(),
        // }))

        Ok(Json(UserResponse {
            username: user.username.unwrap(),
            id: user.id.unwrap(),
            token: String::from("Hello World!"),
        }))

    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

/// Hashes the password before storage in the database.
fn hash_password(password: String) -> Result<String, StatusCode> {
    bcrypt::hash(password, 10).map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)
}

// Verifies the password in the request matches that of the database.
fn verify_password(password: String, hash: &str) -> Result<bool, StatusCode> {
    bcrypt::verify(password, hash).map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)
}
