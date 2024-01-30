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

use crate::stores::user_store::UserStore;
use crate::styles::STYLESHEET;

/// The response from the backend API containing a list of missions.
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct MissionsList {
    pub missions: Option<Vec<Mission>>,
}

/// The page of the web application that houses user missions.
#[styled_component(Missions)]
pub fn missions() -> Html {
    let stylesheet = Style::new(STYLESHEET).expect("Failed to create style");

    // Scroll to top of page on load
    scroll_to_top();

    // Use Yewdux to hold missions
    let (mission_list_store, _mission_list_dispatch) = use_store::<MissionListStore>();

    // Use Yewdux to get user information
    let (user_store, user_dispatch) = use_store::<crate::stores::user_store::UserStore>();

    // Use base_url to send requests to the backend API
    let base_url = use_context::<String>().expect("Context not found");
    let base_url_clone = base_url.clone();

    // Callback for logout button
    let onclick: Callback<MouseEvent> = user_dispatch.reduce_mut_callback(move |user_store| {
        let base_url = base_url_clone.clone();
        let token = user_store
            .token
            .clone()
            .expect("Logged in user has no token");

        // Spawn a new thread
        wasm_bindgen_futures::spawn_local(async move {
            // Send a POST request to the backend API to logout the user
            let response = Request::post(&format!("{}/users/logout", base_url))
                .header("content-type", "application/json")
                .header("authorization", &format!("Bearer {}", token))
                .send()
                .await
                .unwrap();

            match response.status() {
                200 => {
                    // Clear the user_store
                    let user_dispatch = Dispatch::<UserStore>::new();

                    user_dispatch.reduce_mut(|user_store| {
                        user_store.token = None;
                        user_store.id = None;
                        user_store.username = None;
                    });
                }
                _ => {
                    // Log a message to the console
                    web_sys::console::log_1(&"Error logging out".into());
                }
            }
        });
    });

    // Request the user's missions on load
    let user_store_clone = user_store.clone();
    use_effect_with_deps(
        move |_| {
            if user_store_clone.token.is_some() {
            // Spawn a new thread
            wasm_bindgen_futures::spawn_local(async move {
                // Get the user_id and token from the user_store
                let user_id = user_dispatch.get().id.expect("Logged in user has no id");
                let token = user_dispatch.get().token.clone().expect("Logged in user has no token");

                // Send a GET request to the backend API to get all missions
                let response = Request::get(&format!("{}/users/{}", base_url, user_id))
                    .header("content-type", "application/json")
                    .header("authorization", &format!("Bearer {}", token))
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
                            &format!("Error retrieving missions.").into(),
                        );
                    }
                }
            });}
        },
        (),
    );

    html!(
        <div class={stylesheet}>
            <div class={"missions"}>
                <NavBar />
                if let Some(username) = &user_store.username {
                    <header>
                        <h1 class={"user-welcome"}>{format!("Mission Log: {}", username)}</h1>
                    </header>
                }  else {
                    <h1> {"Mission Log"} </h1>
                }
                    // If the user is logged in, show the missions
                    if let Some(_) = &user_store.username {
                        <div class={"logged-in"}>
                            <div class="new-mission-container">
                                <button {onclick}>{"Log Out"}</button>
                                <Link<Route> to={Route::NewMission}>
                                    {"Create New Mission"}
                                </Link<Route>>
                            </div>
                            // Check if there are any missions
                            if let Some(missions) = &mission_list_store.missions {
                                {if missions.len() != 0 {
                                    html!(<h2>{"Your Missions:"}</h2>)
                                } else {
                                    html!(<></>)
                                }}
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
