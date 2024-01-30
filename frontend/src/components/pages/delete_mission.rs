//! Webpage for deleting a mission.

use reqwasm::http::Request;
use stylist::{yew::styled_component, Style};

use yew::prelude::*;
use yew_router::{components::Link, hooks::use_navigator};
use yewdux::dispatch::Dispatch;

use crate::{
    components::{
        pages::scroll_to_top,
        subcomponents::{contact_footer::ContactFooter, nav_bar::NavBar},
    }, router::Route, stores::user_store::UserStore
};

use crate::styles::STYLESHEET;

/// The properties of the DeleteMission component. Used to delete a mission in the database.
#[derive(Properties, PartialEq)]
pub struct Props {
    pub mission_id: i32,
}

/// The page of the web application that allows users to delete a mission.
#[styled_component(DeleteMission)]
pub fn delete_mission(props: &Props) -> Html {
    let stylesheet = Style::new(STYLESHEET).expect("Failed to create style");

    // Scroll to top of page on load
    scroll_to_top();

    // Use navigator to redirect the user after a successful deletion
    let navigator = use_navigator().unwrap();

    // Get the mission ID from the properties
    let mission_id = props.mission_id;

    // Use base_url to send requests to the backend API
    let base_url = use_context::<String>().expect("Context not found");

    // Callback for delete form submission
    let onsubmit: Callback<SubmitEvent> = Callback::from(move |event: SubmitEvent| {
        // Prevent the default form submission behavior
        event.prevent_default();

        let navigator = navigator.clone();
        let base_url = base_url.clone();

        wasm_bindgen_futures::spawn_local(async move {
            // Get the user token from local storage
            let user_dispatch = Dispatch::<UserStore>::new();
            let token = user_dispatch.get().token.clone().expect("Logged in user has no token");

            // Send a DELETE request to the backend API to delete the mission from the database
            let response = Request::delete(&format!("{}/missions/{}", base_url, mission_id))
                .header("content-type", "application/json")
                .header("authorization", &format!("Bearer {}", token))
                .send()
                .await
                .unwrap();

            match response.status() {
                200 => {
                    // Redirect to the missions page
                    navigator.push(&Route::Missions);
                },

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

                _ => {
                    // Failed to delete the mission
                    navigator.push(&Route::DisplayError {
                        error_message: "Failed to delete the mission".to_string(),
                    });
                }
            }
        });
    });

    html!(
        <div class={stylesheet}>
            <div class={"delete"}>
                <NavBar />
                <div class={"delete-content"}>
                    <h1>{"Are you sure you want to delete this mission?"}</h1>
                    <form class={"delete-btn-container"} {onsubmit}>
                        <Link<Route> to={Route::Missions}>
                            {"Cancel"}
                        </Link<Route>>
                        <button class={"delete-button"} type="submit">{"Delete"}</button>
                    </form>
                </div>
                <ContactFooter />
            </div>
        </div>
    )
}
