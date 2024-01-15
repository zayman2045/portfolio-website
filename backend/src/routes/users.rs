//! Handles user specific routes.

use axum::http::StatusCode;
use axum::{Extension, Json};
use sea_orm::*;
use serde::{Deserialize, Serialize};

use crate::entities::prelude::*;
use crate::entities::users;

#[derive(Deserialize, Serialize)]
pub struct RequestUser {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct ResponseUser {
    pub username: String,
    pub token: String,
    pub message: String,
}

/// Creates a new user in the database.
pub async fn create_user(
    Extension(database): Extension<DatabaseConnection>,
    Json(user): Json<RequestUser>,
) -> Result<Json<ResponseUser>, StatusCode> {
    let new_user = users::ActiveModel {
        username: ActiveValue::Set(user.username.clone()),
        password: ActiveValue::Set(user.password),
        ..Default::default()
    };

    // Check if the user already exists
    match Users::find()
        .filter(users::Column::Username.eq(user.username.clone()))
        .one(&database)
        .await
    {
        Ok(user) => {
            // If the user already exists, return a conflict error
            if user.is_some() {
                return Err(StatusCode::CONFLICT);
            }
        }
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    // Insert the new user into the database, return an error if it fails
    match Users::insert(new_user).exec(&database).await {
        Ok(_) => {
            return Ok(Json(ResponseUser {
                username: user.username,
                token: "test_token".to_string(),
                message: "Sign Up Successful".to_string(),
            }))
        }
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
}

/// TODO: Gets a user from the database.
pub async fn get_user(Extension(_database): Extension<DatabaseConnection>) -> Json<ResponseUser> {
    Json(ResponseUser {
        username: "some_user".to_string(),
        token: "test_token".to_string(),
        message: "Log In Successful".to_string(),
    })
}
