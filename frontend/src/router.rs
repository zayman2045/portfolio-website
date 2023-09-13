use crate::pages::{hello_server::HelloServer, home::Home};
use yew::prelude::*;
use yew_router::prelude::*;

// Create yew routes that will be requested by the browser
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/hello-server")]
    HelloServer,
}

// Dispatch routes and render functional components
pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::HelloServer => html! { <HelloServer /> },
    }
}
