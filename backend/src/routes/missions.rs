//! Handles mission specific routes.

use axum::http::StatusCode;
use axum::{extract::Extension, Json};
use sea_orm::{entity::*, DatabaseConnection, QueryFilter};
use serde::{Deserialize, Serialize};

use crate::entities::prelude::*;
use crate::entities::missions;


#[derive(Deserialize, Serialize)]
pub struct Mission {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub content: String,
}

/// The request body for creating a new mission.
#[derive(Deserialize, Serialize)]
pub struct RequestMissionCreate {
    pub user_id: i32,
    pub title: String,
    pub content: String,
}

/// The response body for creating a new mission.
#[derive(Deserialize, Serialize)]
pub struct ResponseMissionCreate {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub content: String,
}

/// The request body for getting all missions.
#[derive(Deserialize, Serialize)]
pub struct RequestAllMissions {
    pub user_id: i32,
}

/// The response body for getting all missions.
#[derive(Deserialize, Serialize)]
pub struct ResponseAllMissions {
    pub missions: Vec<Mission>,
}

/// Creates a new mission in the database.
pub async fn create_mission(
    Extension(database): Extension<DatabaseConnection>,
    Json(request_mission): Json<RequestMissionCreate>,
) -> Result<Json<ResponseMissionCreate>, StatusCode> {
    // Create a new mission model
    let new_mission = missions::ActiveModel {
        user_id: ActiveValue::Set(request_mission.user_id),
        title: ActiveValue::Set(request_mission.title.clone()),
        content: ActiveValue::Set(Some(request_mission.content.clone())),
        ..Default::default()
    };

    // Insert the new mission into the database
    match Missions::insert(new_mission).exec(&database).await {
        Ok(insert_result) => {
            return Ok(Json(ResponseMissionCreate {
                id: insert_result.last_insert_id,
                user_id: request_mission.user_id,
                title: request_mission.title,
                content: request_mission.content,
            }))
        }
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
}

/// Gets all missions for a user.
pub async fn get_missions(
    Extension(database): Extension<DatabaseConnection>,
    Json(request_missions): Json<RequestAllMissions>,
) -> Result<Json<ResponseAllMissions>, StatusCode> {
    // Get all missions for the user
    match Missions::find()
        .filter(missions::Column::UserId.eq(request_missions.user_id))
        .all(&database)
        .await
    {
        Ok(missions) => {
            // Return the missions
            return Ok(Json(ResponseAllMissions {
                missions: missions
                    .into_iter()
                    .map(|mission| Mission {
                        id: mission.id,
                        user_id: mission.user_id,
                        title: mission.title,
                        content: mission.content.unwrap_or(String::from("")),
                    })
                    .collect(),
            }))
        }
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
}