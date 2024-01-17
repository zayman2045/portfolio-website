//! Webpage for deleting a mission.

use stylist::{yew::styled_component, Style};

use yew::prelude::*;

use crate::components::{
    pages::scroll_to_top,
    subcomponents::{contact_footer::ContactFooter, nav_bar::NavBar},
};

const STYLE_FILE: &str = include_str!("stylesheets/styles.css");

/// The properties of the DeleteMission component. Used to delete a mission in the database.
#[derive(Properties, PartialEq)]
pub struct Props {
    pub mission_id: i32
}

/// The page of the web application that allows users to inspect a mission.
#[styled_component(DeleteMission)]
pub fn delete_mission(props: &Props) -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();

    // Scroll to top of page on load
    scroll_to_top();

    html!(
        <div class={stylesheet}>
            <div class={"delete"}>
                <NavBar />

                <h1>{"Delete Mission Page Here!"}</h1>

                <ContactFooter />
            </div>
        </div>
    )
}