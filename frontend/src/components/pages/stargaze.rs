//! Webpage for the Stargaze project.

use stylist::{yew::styled_component, Style};
use yew::prelude::*;

use crate::components::pages::scroll_to_top;
use crate::components::subcomponents::contact_footer::ContactFooter;
use crate::components::subcomponents::nav_bar::NavBar;

const STYLE_FILE: &str = include_str!("stylesheets/styles.css");

/// The page of the web application that describes the Stargaze project.
#[styled_component(Stargaze)]
pub fn stargaze() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();

    // Scroll to top of page on load
    scroll_to_top();

    html!(
        <div class={stylesheet}>
            <div class={"stargaze"}>
                <NavBar />
                <header>
                    <h1> {"Stargaze"} </h1>
                    <p>{"A 2D game developed using the Bevy game engine in Rust, showcasing modern game development techniques and performance optimization."}</p>
                </header>
                <ContactFooter />
            </div>
        </div>
    )
}
