//! Webpage for user missions.

use stylist::{yew::styled_component, Style};
use yew::prelude::*;
use yew_router::components::Link;

use crate::components::pages::scroll_to_top;
use crate::components::subcomponents::contact_footer::ContactFooter;
use crate::components::subcomponents::nav_bar::NavBar;
use crate::router::Route;

const STYLE_FILE: &str = include_str!("stylesheets/styles.css");

/// Represents the page of the web application that houses user missions.
#[styled_component(Missions)]
pub fn missions() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();

    // Scroll to top of page on load
    scroll_to_top();

    html!(
        <div class={stylesheet}>
            <div class={"missions"}>
                <NavBar />
                <header>
                    <h1> {"Mission Log"} </h1>
                    </header>
                    // Conditionally render the login and signup links if the user is not logged in
                    <div class={"logged-out"}>
                        <h2>{"From this terminal you will be able to:"}</h2>
                        <ul>
                            <li>{"Commence new missions and detail their objectives."}</li>
                            <li>{"Retrieve pre-existing mission data upon logging in."}</li>
                            <li>{"Uplink new data to ongoing missions, including new information and status changes."}</li>
                            <li>{"Decommission active missions, removing them from the log."}</li>
                        </ul>
                        <h2>{"You must be logged in to view your missions."}</h2>
                        <div class="btn-container">
                            <Link<Route> to={Route::Login}>
                                {"Log In"}
                            </Link<Route>>
                            <Link<Route> to={Route::Signup}>
                                {"Sign Up"}
                            </Link<Route>>
                        </div>
                    </div>
                <ContactFooter />
            </div>
        </div>
    )
}
