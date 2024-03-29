//! Entry point for the frontend.

use frontend::{App, AppProps};
use reqwasm::http::Request;
use wasm_bindgen_futures::spawn_local;
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    api_base_url: String,
}

/// Fetches the api base url from config.json.
async fn fetch_config() -> Result<Config, reqwasm::Error> {
    let response = Request::get("/config.json").send().await?;
    let config = response.json().await?;
    Ok(config)
}



/// Renders the App component.
fn main() {
    spawn_local(async {
        match fetch_config().await {
            Ok(config) => {
                yew::Renderer::<App>::with_props(AppProps {
                    api_base_url: config.api_base_url,
                })
                .render();
            }
            Err(error) => {
                web_sys::console::error_1(&format!("Failed to fetch config: {:?}", error).into());
            }
        }
    });

}

