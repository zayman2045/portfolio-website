//! Webpage for inspecting a mission.

use stylist::{yew::styled_component, Style};

use yew::prelude::*;
use yew_router::components::Link;

use crate::{components::{
    pages::scroll_to_top,
    subcomponents::{contact_footer::ContactFooter, nav_bar::NavBar},
}, router::Route};

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

    html!(
        <div class={stylesheet}>
            <div class={"inspect"}>
                <NavBar />
                <h1>{"Inspect Mission Page Here!"}</h1>
                <h2>{format!("Mission ID: {}", props.mission_id)}</h2>
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
