use crate::components::organisms::{
    ello::Ello, funder::Funder, home::Home, login::Login,
    signup::Signup, stargaze::Stargaze,
};

use yew::prelude::*;
use yew_router::prelude::*;

// Create yew routes that will be requested by the browser
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
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
    }
}
