//! Webpage for deleting a mission.

use reqwasm::http::Request;
use stylist::{yew::styled_component, Style};

use yew::prelude::*;
use yew_router::{components::Link, hooks::use_navigator};

use crate::{
    components::{
        pages::scroll_to_top,
        subcomponents::{contact_footer::ContactFooter, nav_bar::NavBar},
    },
    router::Route,
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

    // Callback for delete form submission
    let onsubmit: Callback<SubmitEvent> = Callback::from(move |event: SubmitEvent| {
        // Prevent the default form submission behavior
        event.prevent_default();

        let navigator = navigator.clone();

        wasm_bindgen_futures::spawn_local(async move {
            // Send a DELETE request to the backend API to delete the mission from the database
            let response = Request::delete(&format!("/api/missions/{}", mission_id))
                .header("content-type", "application/json")
                .send()
                .await
                .unwrap();

            match response.status() {
                200 => {
                    // Redirect to the missions page
                    navigator.push(&Route::Missions);
                }
                _ => {
                    // Failed to delete the mission
                    navigator.push(&Route::DisplayError { error_message: "Failed to delete the mission".to_string() });
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
