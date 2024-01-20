//! Routing functionality for the web application.
//!
//! Contains the definition of all routes in the application and the logic to switch between them.

use crate::components::pages::{
    about_me::AboutMe, about_projects::AboutProjects, about_site::AboutSite,
    build_mission::BuildMission, delete_mission::DeleteMission, display_error::DisplayError,
    ello::Ello, funder::Funder, home::Home, inspect_mission::InspectMission, login::Login,
    missions::Missions, signup::Signup, stargaze::Stargaze,
};

use yew::prelude::*;
use yew_router::prelude::*;

/// The different routes in the application.
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
    #[at("/signup")]
    Signup,
    #[at("/signup/user-exists")]
    SignupUserExists,
    #[at("/signup/password-mismatch")]
    SignupPasswordMismatch,
    #[at("/signup/error")]
    SignupError,
    #[at("/login")]
    Login,
    #[at("/login/error")]
    LoginError,
    #[at("/login/invalid")]
    LoginInvalid,
    #[at("/missions")]
    Missions,
    #[at("/missions/new")]
    NewMission,
    #[at("/missions/:mission_id")]
    InspectMission { mission_id: i32 },
    #[at("/missions/:mission_id/edit")]
    EditMission { mission_id: i32 },
    #[at("/missions/:mission_id/delete")]
    DeleteMission { mission_id: i32 },
    #[at("/error/:error_message")]
    DisplayError { error_message: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

/// Returns the component corresponding to the given route.
///
/// This function takes a `Route` and returns the Yew component for the corresponding page.
/// It is used to render the correct page when the route changes.
pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::AboutMe => html! {<AboutMe />},
        Route::AboutSite => html! {<AboutSite />},
        Route::AboutProjects => html! {<AboutProjects />},
        Route::Stargaze => html! { <Stargaze /> },
        Route::Funder => html! { <Funder /> },
        Route::Ello => html! { <Ello /> },
        Route::Signup => html! {<Signup />},
        Route::SignupUserExists => {
            html! {<Signup message={Some("Username already exists.".to_string())} />}
        }
        Route::SignupPasswordMismatch => {
            html! {<Signup message={Some("Passwords do not match.".to_string())} />}
        }
        Route::SignupError => html! {<Signup message={Some("An error occurred.".to_string())} />},
        Route::Login => html! {<Login />},
        Route::LoginInvalid => {
            html! {<Login message={Some("Invalid username or password.".to_string())} />}
        }
        Route::LoginError => html! {<Login message={Some("An error occurred.".to_string())} />},
        Route::Missions => html! {<Missions />},
        Route::NewMission => html! {<BuildMission />},
        Route::InspectMission { mission_id } => html! {<InspectMission mission_id={mission_id} />},
        Route::EditMission { mission_id } => html! {<BuildMission mission_id={Some(mission_id)} />},
        Route::DeleteMission { mission_id } => html! {<DeleteMission mission_id={mission_id} />},
        Route::DisplayError { error_message } => {
            html! {<DisplayError error_message={error_message}/>}
        }
        Route::NotFound => html! {<div>{"404 - Not Found"}</div>},
    }
}
