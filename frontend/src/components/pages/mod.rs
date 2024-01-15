//! Contains the different pages of the application. Each sub-module represents a different page.

pub mod about_me;
pub mod about_projects;
pub mod about_site;
pub mod ello;
pub mod funder;
pub mod home;
pub mod login;
pub mod signup;
pub mod stargaze;

use web_sys::window;

/// Scrolls to the top of the page.
fn scroll_to_top() {
    if let Some(window) = window() {
        window.scroll_to_with_x_and_y(0.0, 0.0);
    }
}