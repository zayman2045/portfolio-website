//! Webpage for inspecting a mission.

use reqwasm::http::Request;
use stylist::{yew::styled_component, Style};

use yew::prelude::*;
use yew_router::components::Link;
use yewdux::{functional::use_store, dispatch::Dispatch};

use crate::{components::{
    pages::scroll_to_top,
    subcomponents::{contact_footer::ContactFooter, nav_bar::NavBar},
}, router::Route, stores::mission_store::MissionStore};

const STYLE_FILE: &str = include_str!("stylesheets/styles.css");

/// The properties of the InspectMission component. Used to request a mission from the server.
#[derive(Properties, PartialEq)]
pub struct Props {
    pub mission_id: i32
}

/// The page of the web application that allows users to inspect a mission.
#[styled_component(InspectMission)]
pub fn inspect_mission(props: &Props) -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();

    // Scroll to top of page on load
    scroll_to_top();

    let mission_id = props.mission_id;

    // Use Yewdux to hold mission
    let (mission_store, _mission_dispatch) = use_store::<MissionStore>();

    use_effect_with_deps(
        move |_| {
            // Spawn a new thread
            wasm_bindgen_futures::spawn_local(async move {


                // Send a GET request to the backend API to get the mission
                let response = Request::get(&format!("/api/missions/{}", mission_id))
                    .header("content-type", "application/json")
                    .send()
                    .await
                    .unwrap();

                match response.status() {
                    // Successfully retrieved missions
                    200 => {
                        // Convert the response to a of mission
                        let mission: MissionStore = response.json().await.unwrap();

                        // Use Yewdux store to hold mission
                        let mission_dispatch = Dispatch::<MissionStore>::new();
                        mission_dispatch.reduce_mut( |mission_store| {
                            mission_store.id = mission.id;
                            mission_store.user_id = mission.user_id;
                            mission_store.title = mission.title;
                            mission_store.content = mission.content;
                        });

                    }

                    // Error retrieving mission
                    _ => {
                        // Log a message to the console
                        web_sys::console::log_1(
                            &format!("Error retrieving missions: {:?}", response).into(),
                        );
                    }
                }
            });
        },
        (),
    );

    html!(
        <div class={stylesheet}>
            <div class={"inspect"}>
                <NavBar />
                <h1>{"Inspect Mission Page Here!"}</h1>
                <h2>{&mission_store.title}</h2>
                <p>{mission_store.content.as_ref().unwrap_or(&"No content".to_string())}</p>
                <Link<Route> to={Route::EditMission {mission_id: props.mission_id}}>
                                    {"Edit Mission"}
                </Link<Route>>
                <Link<Route> to={Route::DeleteMission {mission_id: props.mission_id}}>
                                    {"Delete Mission"}
                </Link<Route>>
                <ContactFooter />
            </div>
        </div>
    )
}
