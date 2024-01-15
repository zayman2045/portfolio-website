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
    pub id: i32,
    pub token: String,
}

/// Creates a new user in the database.
pub async fn create_user(
    Extension(database): Extension<DatabaseConnection>,
    Json(user): Json<RequestUser>,
) -> Result<Json<ResponseUser>, StatusCode> {
    // Create a new user model
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
            // Return a conflict status code if the user is found
            if user.is_some() {
                return Err(StatusCode::CONFLICT);
            } else {
                // Insert the new user into the database
                match Users::insert(new_user).exec(&database).await {
                    Ok(_) => {
                        return Ok(Json(ResponseUser {
                            username: user.clone().unwrap().username,
                            id: user.clone().unwrap().id,
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
    Json(request_user): Json<RequestUser>,
) -> Result<Json<ResponseUser>, StatusCode> {
    // Check if the user exists
    match Users::find()
        .filter(users::Column::Username.eq(request_user.username.clone()))
        .one(&database)
        .await
    {
        Ok(database_user) => {
            // Return a not found status code if the user is not found
            if database_user.is_none() {
                return Err(StatusCode::NOT_FOUND);
            } else {
                // Check if the password is correct
                if database_user.clone().unwrap().password == request_user.password {
                    return Ok(Json(ResponseUser {
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
