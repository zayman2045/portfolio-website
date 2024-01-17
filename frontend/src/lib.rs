//! Core functionality for the client-side web application.
//!
//! Contains the primary application logic and sets up the application environment,
//! including the initialization of various components, routers, and stores.

pub mod components;
pub mod router;
pub mod stores;
pub mod styles;

use router::{switch, Route};
use yew::prelude::*;
use yew_router::prelude::*;

/// The main application component.
///
/// This functional component sets up the application environment, initializes the router and starts the rendering loop.
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}