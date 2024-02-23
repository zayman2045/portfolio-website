//! Core functionality for the client-side web application.
//!
//! Contains the primary application logic and sets up the application environment,
//! including the initialization of various components, routers, and stores.

pub mod components;
pub mod router;
pub mod stores;
mod styles;

use router::{switch, Route};
use yew::prelude::*;
use yew_router::prelude::*;

/// The properties of the App component.
#[derive(Properties, Clone, PartialEq)]
pub struct AppProps {
    pub api_base_url: String,
}

/// The main application component.
///
/// This functional component sets up the application environment, initializes the router and starts the rendering loop.
#[function_component(App)]
pub fn app(props: &AppProps) -> Html {
    let base_url = use_memo(|_| props.api_base_url.clone(), ());
    
    html! {
        <ContextProvider<String> context={(*base_url).clone()}>
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
        </ContextProvider<String>>
    }
}
