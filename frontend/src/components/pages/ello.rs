//! Webpage for the Ello project.

use stylist::{yew::styled_component, Style};
use yew::prelude::*;

use crate::components::pages::scroll_to_top;
use crate::components::subcomponents::contact_footer::ContactFooter;
use crate::components::subcomponents::nav_bar::NavBar;

const STYLE_FILE: &str = include_str!("stylesheets/styles.css");

/// Represents the page of the web application that describes the Ello project.
#[styled_component(Ello)]
pub fn ello() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();

    // Scroll to top of page on load
    scroll_to_top();

    html!(
        <div class={stylesheet}>
            <div class={"ello"}>
                <NavBar />
                <header>
                    <h1> {"Ello"} </h1>
                    <p>{"A chatbot powered by the OpenAI GPT-3 Turbo model, designed to help users practice simple conversations in a chosen language."}</p>
                </header>
                <ContactFooter />
            </div>
        </div>
    )
}
