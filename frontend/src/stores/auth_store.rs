//! Stores the authentication information of a user.

use yewdux::prelude::*;

/// Represents the authentication information of a user.
#[derive(Debug, Default, Clone, PartialEq, Eq, Store)]
pub struct AuthStore {
    pub username: Option<String>,
    pub password: Option<String>,
    pub confirmed_password: Option<String>,
}