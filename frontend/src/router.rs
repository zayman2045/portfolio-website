use crate::components::organisms::{hello_server::HelloServer, home::Home, stargaze::Stargaze, funder::Funder, ello::Ello};
use yew::prelude::*;
use yew_router::prelude::*;

// Create yew routes that will be requested by the browser
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/hello-server")]
    HelloServer,
    #[at("/stargaze")]
    Stargaze,
    #[at("/funder")]
    Funder,
    #[at("/ello")]
    Ello
}

// Dispatch routes and render functional components
pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::HelloServer => html! { <HelloServer /> },
        Route::Stargaze => html! { <Stargaze /> },
        Route::Funder => html! { <Funder /> },
        Route::Ello => html! { <Ello /> },
    }
}
