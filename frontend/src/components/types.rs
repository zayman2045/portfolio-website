//! This module contains types that are used in the frontend.

use serde::{Deserialize, Serialize};

/// The response from the backend API when a user is created or logged in.
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct ResponseUser {
    pub username: Option<String>,
    pub id: Option<i32>,
    pub token: Option<String>,
}
