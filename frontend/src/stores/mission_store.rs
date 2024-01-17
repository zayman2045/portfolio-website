//! Stores user's list of missions.

use serde::{Deserialize, Serialize};
use yewdux::prelude::*;



/// A single mission.
#[derive(Debug, Default, Clone, PartialEq, Eq, Store, Serialize, Deserialize)]
pub struct MissionStore {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub content: Option<String>,
}

/// The user's list of missions.
#[derive(Debug, Default, Clone, PartialEq, Eq, Store)]
pub struct MissionListStore {
    pub missions: Option<Vec<Mission>>,
}

/// Helper type for the user's list of missions.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Mission {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub content: Option<String>,
}