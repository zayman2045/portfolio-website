//! Webpage for user missions.

use stylist::{yew::styled_component, Style};
use yew::prelude::*;

use crate::components::pages::scroll_to_top;
use crate::components::subcomponents::contact_footer::ContactFooter;
use crate::components::subcomponents::nav_bar::NavBar;

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
                    <h1> {"Missions"} </h1>
                    <p>{"Working so far..."}</p>
                </header>
                <ContactFooter />
            </div>
        </div>
    )
}