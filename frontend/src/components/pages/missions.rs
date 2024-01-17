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
use crate::stores::mission_store::{MissionStore, Mission};

const STYLE_FILE: &str = include_str!("stylesheets/styles.css");

/// Represents the properties of the Signup component.
#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub user_id: Option<i32>,
    pub username: Option<String>,
}

/// Represents the response from the backend API containing a list of missions.
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct MissionsList {
    pub missions: Option<Vec<Mission>>,
}

/// Represents the page of the web application that houses user missions.
#[styled_component(Missions)]
pub fn missions(props: &Props) -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();

    // Scroll to top of page on load
    scroll_to_top();

    // Use Yewdux to hold missions 
    let (missions_store, _missions_dispatch) = use_store::<MissionStore>();

        use_effect_with_deps(move |_| {

        // Spawn a new thread
        wasm_bindgen_futures::spawn_local(async move {

            // Send a GET request to the backend API to get all missions
            let response = Request::get("/api/users/2")
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
                    let missions_dispatch = Dispatch::<MissionStore>::new();
                    missions_dispatch.reduce_mut(|missions_store| {
                        missions_store.missions = missions.missions;
                    });
                }

                // Error retrieving missions
                _ => {
                    // Log a message to the console
                    web_sys::console::log_1(&format!("Error retrieving missions: {:?}", response).into());
                }
            }
        });
    }, ());

    html!(
        <div class={stylesheet}>
            <div class={"missions"}>
                <NavBar />
                <header>
                    <h1> {"Mission Log"} </h1>
                    </header>
                    // Conditionally render the login and signup links if the user is not logged in
                    if let Some(_username) = &props.username {
                        <div class={"logged-in"}>
                            <div class="new-mission-container">
                                <Link<Route> to={Route::Home}>
                                    {"Create New Mission"}
                                </Link<Route>>
                            </div>
                            if let Some(_missions) = &missions_store.missions {
                                <div class="mission-container">
                                <Link<Route> to={Route::Home}>
                                    {"Found Mission!"}
                                </Link<Route>>
                            </div>
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
