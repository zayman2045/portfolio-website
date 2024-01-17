//! Stores user's list of missions.

use serde::{Serialize, Deserialize};
use yewdux::prelude::*;

/// Represents the user's list of missions.
#[derive(Debug, Default, Clone, PartialEq, Eq, Store)]
pub struct MissionStore {
    pub missions: Option<Vec<Mission>>
}

/// Represents a single mission.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Mission {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub content: Option<String>,
}


