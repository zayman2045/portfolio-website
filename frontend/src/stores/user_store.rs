//! Stores the user's username and token for authentication

use serde::{Serialize, Deserialize};
use yewdux::prelude::*;

/// Represents the user during a session.
#[derive(Debug, Default, Clone, PartialEq, Eq, Store, Serialize, Deserialize)]
pub struct UserStore {
    pub username: Option<String>,
    pub token: Option<String>,
    pub message: Option<String>
}