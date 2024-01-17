//! Stores the mission data before a mission creating or mission editing form submission.

use yewdux::prelude::*;

/// The mission information needed for form submission.
#[derive(Debug, Default, Clone, PartialEq, Eq, Store)]
pub struct BuildStore {
    pub user_id: Option<i32>,
    pub title: Option<String>,
    pub content: Option<String>,
}