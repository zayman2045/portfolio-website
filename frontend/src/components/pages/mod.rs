//! Contains the different pages of the application. Each sub-module represents a different page.

pub mod about_me;
pub mod about_projects;
pub mod tech_stack;
pub mod home;
pub mod login;
pub mod signup;
pub mod missions;
pub mod build_mission;
pub mod inspect_mission;
pub mod delete_mission;
pub mod display_error;

use web_sys::window;

/// Scrolls to the top of the page.
fn scroll_to_top() {
    if let Some(window) = window() {
        window.scroll_to_with_x_and_y(0.0, 0.0);
    }
}