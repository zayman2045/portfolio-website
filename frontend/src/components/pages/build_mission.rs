//! Webpage for creating or updating a mission.

use reqwasm::http::Request;
use serde_json::json;
use stylist::{yew::styled_component, Style};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::hooks::use_navigator;
use yewdux::dispatch::Dispatch;

use crate::{
    components::{
        pages::scroll_to_top,
        subcomponents::{contact_footer::ContactFooter, nav_bar::NavBar},
    },
    router::Route,
    stores::{build_store::BuildStore, user_store::UserStore},
};

use crate::styles::STYLESHEET;

/// The properties of the BuildMission component. Used to determine if the user is creating a new mission or editing an existing one.
#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub mission_id: Option<i32>,
}

/// The page of the web application that allows users to create or edit a mission.
#[styled_component(BuildMission)]
pub fn build_mission(props: &Props) -> Html {
    let stylesheet = Style::new(STYLESHEET).expect("Failed to create style");

    // Scroll to top of page on load
    scroll_to_top();

    // Use Yewdux to access mission build state
    let build_dispatch = Dispatch::<BuildStore>::new();

    // Store title when onchange event occurs to the title input field
    let onchange_title = build_dispatch.reduce_mut_callback_with(|build_store, event: Event| {
        let title = event.target_unchecked_into::<HtmlInputElement>().value();
        build_store.title = if title.is_empty() { None } else { Some(title) }
    });

    // Store content when onchange event occurs to the content input field
    let onchange_content = build_dispatch.reduce_mut_callback_with(|build_store, event: Event| {
        let content = event.target_unchecked_into::<HtmlInputElement>().value();
        build_store.content = if content.is_empty() {
            None
        } else {
            Some(content)
        }
    });

    // Use navigator to redirect the user after a form submission
    let navigator = use_navigator().unwrap();

    // Clone the mission ID from the properties
    let mission_id = props.mission_id;

    // Use base_url to send requests to the backend API
    let base_url = use_context::<String>().expect("Context not found");

    // Handler for form submission
    let onsubmit =
        build_dispatch.reduce_mut_callback_with(move |build_store, event: SubmitEvent| {
            event.prevent_default();

            let build_store = build_store.clone();
            let navigator = navigator.clone();
            let base_url = base_url.clone();
            let user_dispatch = Dispatch::<UserStore>::new();

            // Spawn a new thread
            wasm_bindgen_futures::spawn_local(async move {
                let build_request = json!({
                    "user_id": user_dispatch.get().id.unwrap(),
                    "title": build_store.title.unwrap(),
                    "content": build_store.content.unwrap_or_default(),
                })
                .to_string();

                let token = user_dispatch
                    .get()
                    .token
                    .clone()
                    .expect("Logged in user has no token");

                if let Some(mission_id) = mission_id {
                    // Send a POST request to the backend API to update the mission
                    let response = Request::post(&format!("{}/missions/{}", base_url, mission_id))
                        .body(build_request)
                        .header("content-type", "application/json")
                        .header("authorization", &format!("Bearer {}", token))
                        .send()
                        .await
                        .unwrap();

                    match response.status() {
                        // Successfully created the mission. Redirect the user to their mission page
                        200 => {
                            navigator.push(&Route::Missions);
                        }

                        // Unauthorized
                        401 => {
                            navigator.push(&Route::DisplayError {
                                error_message: "Unauthorized".to_string(),
                            });
                        }

                        // Forbidden (Token expired)
                        403 => {
                            let user_dispatch = Dispatch::<UserStore>::new();

                            user_dispatch.reduce_mut(|user_store| {
                                user_store.token = None;
                                user_store.id = None;
                                user_store.username = None;
                            });

                            navigator.push(&Route::Missions);
                        },

                        // Failed to update the mission
                        _ => {
                            navigator.push(&Route::DisplayError {
                                error_message: "Internal Server Error".to_string(),
                            });
                        }
                    }
                } else {
                    // Send a POST request to the backend API to create the mission
                    let response = Request::post(&format!("{}/missions", base_url))
                        .body(build_request)
                        .header("content-type", "application/json")
                        .header("authorization", &format!("Bearer {}", token))
                        .send()
                        .await
                        .unwrap();

                    match response.status() {
                        // Successfully created the mission. Redirect the user to their mission page
                        200 => {
                            navigator.push(&Route::Missions);
                        }

                        // Unauthorized
                        401 => {
                            navigator.push(&Route::DisplayError {
                                error_message: "Unauthorized".to_string(),
                            });
                        },

                        // Forbidden (Token expired)
                        403 => {
                            let user_dispatch = Dispatch::<UserStore>::new();

                            user_dispatch.reduce_mut(|user_store| {
                                user_store.token = None;
                                user_store.id = None;
                                user_store.username = None;
                            });

                            navigator.push(&Route::Missions);
                        },

                        // Failed to create the mission
                        _ => {
                            navigator.push(&Route::DisplayError {
                                error_message: "Failed to create the mission".to_string(),
                            });
                        }
                    }
                }
            });
        });

    html!(
        <div class={stylesheet}>
            <div class={"build"}>
                <NavBar />
                <div class={"build-content"}>
                    if let Some(_mission_id) = &props.mission_id {
                        <h1>{"Edit Mission"}</h1>
                    } else {
                        <h1>{"Log a New Mission"}</h1>
                    }
                        <form {onsubmit}>
                        <label for="title">{"Title:"}</label>
                        <input type="text" id="title" placeholder="Title" required=true onchange={onchange_title}/>

                        <label for="content">{"Details:"}</label>
                        <textarea id="content" placeholder="Detail your mission..." onchange={onchange_content}></textarea>

                        <button type="submit">{"Submit"}</button>
                    </form>
                </div>
                <ContactFooter />
            </div>
        </div>
    )
}
