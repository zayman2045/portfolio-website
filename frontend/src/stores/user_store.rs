//! Stores the user's username and token for authentication

use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

/// The user during a session.
#[derive(Debug, Default, Clone, PartialEq, Eq, Store, Serialize, Deserialize)]
#[store(storage = "local", storage_tab_sync)]
pub struct UserStore {
    pub username: Option<String>,
    pub id: Option<i32>,
    pub token: Option<String>,
}
