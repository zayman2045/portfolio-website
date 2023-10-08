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

// Create a new user in the database
pub async fn create_user(
    Extension(database): Extension<DatabaseConnection>,
    Json(user): Json<RequestUser>,
) -> impl IntoResponse {
    let new_user = users::ActiveModel {
        username: ActiveValue::Set(user.username),
        password: ActiveValue::Set(user.password),
        ..Default::default()
    };

    let _result = Users::insert(new_user).exec(&database).await.unwrap();

    Json("User created successfully".to_string())

}

// Get a user from the database
pub async fn get_user(Extension(database): Extension<DatabaseConnection>) -> Json<String> {
    let user = Users::find().one(&database).await.unwrap();
    Json(user.unwrap().username)
}
