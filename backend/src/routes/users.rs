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
        password: ActiveValue::Set(user_request.password),
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
    // Check if the user exists
    match Users::find()
        .filter(users::Column::Username.eq(user_request.username.clone()))
        .one(&database)
        .await
    {
        Ok(database_user) => {
            // Return a not found status code if the user is not found
            if database_user.is_none() {
                return Err(StatusCode::NOT_FOUND);
            } else {
                // Check if the password is correct
                if database_user.clone().unwrap().password == user_request.password {
                    return Ok(Json(UserResponse {
                        username: database_user.clone().unwrap().username,
                        id: database_user.clone().unwrap().id,
                        token: String::from("Hello World!"),
                    }));
                } else {
                    return Err(StatusCode::UNAUTHORIZED);
                }
            }
        }
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
}
