use yewdux::prelude::*;

// Shared state used to identify users
#[derive(Debug, Default, Clone, PartialEq, Eq, Store)]
pub struct UserStore {
    pub username: Option<String>,
    pub is_logged_in: bool
}