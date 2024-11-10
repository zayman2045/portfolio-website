//! Entry point for the frontend.

use frontend::{App, AppProps};
use wasm_bindgen_futures::spawn_local;

const API_BASE_URL: Option<&'static str> = option_env!("API_BASE_URL");

/// Renders the App component.
fn main() {
        let api_base_url = API_BASE_URL.expect("API base url not set").to_string();
        web_sys::console::log_1(&format!("api base url value: {:?}", api_base_url).into());

    spawn_local(async {
        yew::Renderer::<App>::with_props(AppProps {
            api_base_url
        })
        .render();
    });
}
