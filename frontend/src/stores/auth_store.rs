use yewdux::prelude::*;

// Shared state used to authenticate users
#[derive(Debug, Default, Clone, PartialEq, Eq, Store)]
pub struct AuthStore {
    pub username: Option<String>,
    pub password: Option<String>,
    pub confirm_password: Option<String>,
    pub passwords_match: bool
}