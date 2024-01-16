//! Routing functionality for the web application.
//!
//! Contains the definition of all routes in the application and the logic to switch between them.

use crate::components::pages::{
    about_me::AboutMe, about_projects::AboutProjects, about_site::AboutSite, ello::Ello,
    funder::Funder, home::Home, login::Login, signup::Signup, stargaze::Stargaze, missions::Missions
};

use yew::prelude::*;
use yew_router::prelude::*;

/// Represents the different routes in the application.
///
/// Each variant of this enum corresponds to a different page in the application.
/// The `#[at]` attribute defines the path for each route.
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about-me")]
    AboutMe,
    #[at("/about-projects")]
    AboutProjects,
    #[at("/about-site")]
    AboutSite,
    #[at("/stargaze")]
    Stargaze,
    #[at("/funder")]
    Funder,
    #[at("/ello")]
    Ello,
    #[at("/login")]
    Login,
    #[at("/signup")]
    Signup,
    #[at("/signup/user-exists")]
    SignupUserExists,
    #[at("/signup/password-mismatch")]
    SignupPasswordMismatch,
    #[at("/signup/error")]
    SignupError,
    #[at("/missions")]
    Missions,
}

/// Returns the component corresponding to the given route.
///
/// This function takes a `Route` and returns the Yew component for the corresponding page.
/// It is used to render the correct page when the route changes.
pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Stargaze => html! { <Stargaze /> },
        Route::Funder => html! { <Funder /> },
        Route::Ello => html! { <Ello /> },
        Route::Login => html! {<Login />},
        Route::Signup => html! {<Signup />},
        Route::SignupUserExists => html! {<Signup message={Some("Username already exists.".to_string())} />},
        Route::SignupPasswordMismatch => html! {<Signup message={Some("Passwords do not match.".to_string())} />},
        Route::SignupError => html! {<Signup message={Some("An error occurred.".to_string())} />},
        Route::AboutMe => html! {<AboutMe />},
        Route::AboutProjects => html! {<AboutProjects />},
        Route::AboutSite => html! {<AboutSite />},
        Route::Missions => html! {<Missions />},
    }
}