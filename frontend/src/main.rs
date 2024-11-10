//! Entry point for the frontend.

use frontend::{App, AppProps};
use reqwasm::http::Request;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
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

const API_BASE_URL: Option<&'static str> = option_env!("API_BASE_URL");

/// Renders the App component.
fn main() {
        let api_base_url = API_BASE_URL.expect("API base url not set");
        web_sys::console::log_1(&format!("api base url value: {:?}", api_base_url).into());

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
