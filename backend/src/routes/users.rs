use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use sea_orm::EntityTrait;
use sea_orm::{ActiveValue, DatabaseConnection};
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
}

// Create a new user in the database
pub async fn create_user(
    Extension(database): Extension<DatabaseConnection>,
    Json(user): Json<RequestUser>,
) -> Result<Json<ResponseUser>, StatusCode> {
    let new_user = users::ActiveModel {
        username: ActiveValue::Set(user.username.clone()),
        password: ActiveValue::Set(user.password),
        ..Default::default()
    };

    // Insert the new user into the database, return an error if it fails
    match Users::insert(new_user).exec(&database).await {
        Ok(_) => {
            return Ok(Json(ResponseUser {
                username: user.username,
            }))
        }
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
}

// Get a user from the database
pub async fn get_user(Extension(database): Extension<DatabaseConnection>) -> Json<ResponseUser> {
    let _user = Users::find().one(&database).await.unwrap();
    todo!();
}
