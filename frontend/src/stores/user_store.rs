use serde::{Serialize, Deserialize};
use yewdux::prelude::*;

// Shared state used to identify users
#[derive(Debug, Default, Clone, PartialEq, Eq, Store, Serialize, Deserialize)]
pub struct UserStore {
    pub username: Option<String>,
    pub token: Option<String>,
    pub message: Option<String>
}