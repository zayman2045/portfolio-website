//! Webpage for user missions.

use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use stylist::{yew::styled_component, Style};
use yew::prelude::*;
use yew_router::components::Link;
use yewdux::dispatch::Dispatch;
use yewdux::functional::use_store;

use crate::components::pages::scroll_to_top;
use crate::components::subcomponents::contact_footer::ContactFooter;
use crate::components::subcomponents::nav_bar::NavBar;
use crate::router::Route;
use crate::stores::mission_store::{Mission, MissionListStore};

const STYLE_FILE: &str = include_str!("stylesheets/styles.css");

/// The response from the backend API containing a list of missions.
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct MissionsList {
    pub missions: Option<Vec<Mission>>,
}

/// The page of the web application that houses user missions.
#[styled_component(Missions)]
pub fn missions() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();

    // Scroll to top of page on load
    scroll_to_top();

    // Use Yewdux to hold missions
    let (mission_list_store, _mission_list_dispatch) = use_store::<MissionListStore>();

    // Use Yewdux to get user information
    let (user_store, user_dispatch) = use_store::<crate::stores::user_store::UserStore>();

    use_effect_with_deps(
        move |_| {
            // Spawn a new thread
            wasm_bindgen_futures::spawn_local(async move {
                // Get the user_id from the user_store
                let user_id = user_dispatch.get().id.unwrap_or(1);

                // Send a GET request to the backend API to get all missions
                let response = Request::get(&format!("/api/users/{}", user_id))
                    .header("content-type", "application/json")
                    .send()
                    .await
                    .unwrap();

                match response.status() {
                    // Successfully retrieved missions
                    200 => {
                        // Convert the response to a list of missions
                        let missions: MissionsList = response.json().await.unwrap();

                        // Use Yewdux store to hold missions
                        let mission_list_dispatch = Dispatch::<MissionListStore>::new();
                        mission_list_dispatch.reduce_mut(|mission_list_store| {
                            mission_list_store.missions = missions.missions;
                        });
                    }

                    // Error retrieving missions
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
            <div class={"missions"}>
                <NavBar />
                <header>
                    <h1> {"Mission Log"} </h1>
                    </header>
                    // If the user is logged in, show the missions
                    if let Some(username) = &user_store.username {
                        <div class={"logged-in"}>
                            <div class="new-mission-container">
                                <Link<Route> to={Route::NewMission}>
                                    {"Create New Mission"}
                                </Link<Route>>
                            </div>
                            // Check if there are any missions
                            if let Some(missions) = &mission_list_store.missions {
                                <h2>{"Your Missions:"}</h2>
                                {for missions.iter().map(|mission| {
                                    html! {
                                        <div class="mission-container">
                                            <Link<Route> to={Route::InspectMission {mission_id: mission.id}}>
                                                <h3>{ &mission.title }</h3>
                                                <p>{ mission.content.as_ref().unwrap_or(&"No content".to_string()) }</p>
                                            </Link<Route>>
                                        </div>
                                    }
                                })}
                    }
                        </div>
                    } else {
                        <div class={"logged-out"}>
                            <h2>{"From this terminal you will be able to:"}</h2>
                            <ul>
                                <li>{"Commence new missions and detail their objectives."}</li>
                                <li>{"Retrieve pre-existing mission data upon logging in."}</li>
                                <li>{"Uplink new data to ongoing missions, including new information and status changes."}</li>
                                <li>{"Decommission active missions, removing them from the log."}</li>
                            </ul>
                            <h3>{"You must be logged in to view your missions."}</h3>
                            <div class="btn-container">
                                <Link<Route> to={Route::Signup}>
                                    {"Sign Up"}
                                </Link<Route>>
                                <Link<Route> to={Route::Login}>
                                    {"Log In"}
                                </Link<Route>>
                            </div>
                        </div>}
                <ContactFooter />
            </div>
        </div>
    )
}
