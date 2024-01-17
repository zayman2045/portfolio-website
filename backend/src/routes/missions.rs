//! Handles mission specific routes.

use axum::extract::Path;
use axum::http::StatusCode;
use axum::{extract::Extension, Json};
use sea_orm::{entity::*, DatabaseConnection, QueryFilter};
use serde::{Deserialize, Serialize};

use crate::entities::missions;
use crate::entities::prelude::*;

/// The model for a mission.
#[derive(Deserialize, Serialize)]
pub struct Mission {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub content: String,
}

/// The request body for creating a new mission.
#[derive(Deserialize, Serialize)]
pub struct MissionBuildRequest {
    pub user_id: i32,
    pub title: String,
    pub content: Option<String>,
}

/// The response body for creating a new mission.
#[derive(Deserialize, Serialize)]
pub struct MissionBuildResponse {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub content: String,
}

/// The response body for getting all missions for a user.
#[derive(Deserialize, Serialize)]
pub struct MissionListResponse {
    pub missions: Vec<Mission>,
}

/// Creates a new mission in the database.
pub async fn create_mission(
    Extension(database): Extension<DatabaseConnection>,
    Json(request_mission): Json<MissionBuildRequest>,
) -> Result<Json<MissionBuildResponse>, StatusCode> {
    // Create a new mission model
    let new_mission = missions::ActiveModel {
        user_id: ActiveValue::Set(request_mission.user_id),
        title: ActiveValue::Set(request_mission.title.clone()),
        content: ActiveValue::Set(request_mission.content.clone()),
        ..Default::default()
    };

    // Insert the new mission into the database
    match Missions::insert(new_mission).exec(&database).await {
        Ok(insert_result) => {
            return Ok(Json(MissionBuildResponse {
                id: insert_result.last_insert_id,
                user_id: request_mission.user_id,
                title: request_mission.title,
                content: request_mission.content.unwrap_or(String::from("")),
            }))
        }
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
}

/// Lists all missions for a user.
pub async fn list_missions(
    Extension(database): Extension<DatabaseConnection>,
    Path(user_id): Path<u32>,
) -> Result<Json<MissionListResponse>, StatusCode> {
    // Get all missions for the user
    match Missions::find()
        .filter(missions::Column::UserId.eq(user_id))
        .all(&database)
        .await
    {
        Ok(missions) => {
            // Return the missions
            return Ok(Json(MissionListResponse {
                missions: missions
                    .into_iter()
                    .map(|mission| Mission {
                        id: mission.id,
                        user_id: mission.user_id,
                        title: mission.title,
                        content: mission.content.unwrap_or(String::from("")),
                    })
                    .collect(),
            }));
        }
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
}

/// Gets a mission by its ID.
pub async fn get_mission(
    Extension(database): Extension<DatabaseConnection>,
    Path(mission_id): Path<i32>,
) -> Result<Json<Mission>, StatusCode> {
    // Get the mission by its ID
    match Missions::find_by_id(mission_id).one(&database).await {
        Ok(mission) => {
            match mission {
                Some(mission) => {
                    // Return the mission
                    return Ok(Json(Mission {
                        id: mission.id,
                        user_id: mission.user_id,
                        title: mission.title,
                        content: mission.content.unwrap_or(String::from("")),
                    }));
                }
                None => return Err(StatusCode::NOT_FOUND),
            }
        }
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
}

/// Updates a mission by its ID.
pub async fn update_mission(
    Extension(database): Extension<DatabaseConnection>,
    Path(mission_id): Path<i32>,
    Json(request_mission): Json<MissionBuildRequest>,
) -> Result<Json<MissionBuildResponse>, StatusCode> {
    // Get the mission by its ID
    match Missions::find_by_id(mission_id).one(&database).await {
        Ok(mission) => match mission {
            Some(mission) => {
                let mut mission: missions::ActiveModel = mission.into();
                mission.title = ActiveValue::Set(request_mission.title);
                mission.content = ActiveValue::Set(request_mission.content);
                match mission.save(&database).await {
                    Ok(update_result) => match update_result.try_into_model() {
                        Ok(mission) => {
                            return Ok(Json(MissionBuildResponse {
                                id: mission.id,
                                user_id: mission.user_id,
                                title: mission.title,
                                content: mission.content.unwrap_or(String::from("")),
                            }));
                        }
                        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
                    },
                    Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
                }
            }
            None => return Err(StatusCode::NOT_FOUND),
        },
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Deletes a mission by its ID.
pub async fn delete_mission(
    Extension(database): Extension<DatabaseConnection>,
    Path(mission_id): Path<i32>,
) -> Result<(), StatusCode> {
    match Missions::delete_by_id(mission_id).exec(&database).await {
        Ok(_) => return Ok(()),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
