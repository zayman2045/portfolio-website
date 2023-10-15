use crate::components::organisms::{
    about_me::AboutMe, about_projects::AboutProjects, about_site::AboutSite, ello::Ello,
    funder::Funder, home::Home, login::Login, signup::Signup, stargaze::Stargaze,
};

use yew::prelude::*;
use yew_router::prelude::*;

// Create yew routes that will be requested by the browser
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
}

// Dispatch routes and render functional components
pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Stargaze => html! { <Stargaze /> },
        Route::Funder => html! { <Funder /> },
        Route::Ello => html! { <Ello /> },
        Route::Login => html! {<Login />},
        Route::Signup => html! {<Signup />},
        Route::AboutMe => html! {<AboutMe />},
        Route::AboutProjects => html! {<AboutProjects />},
        Route::AboutSite => html! {<AboutSite />},
    }
}
