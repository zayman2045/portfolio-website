//! Webpage for the Funder project.

use stylist::{yew::styled_component, Style};
use yew::prelude::*;

use crate::components::pages::scroll_to_top;
use crate::components::subcomponents::contact_footer::ContactFooter;
use crate::components::subcomponents::nav_bar::NavBar;

use crate::styles::STYLESHEET;

/// The page of the web application that describes the Funder project.
#[styled_component(Funder)]
pub fn funder() -> Html {
    let stylesheet = Style::new(STYLESHEET).expect("Failed to create style");

    // Scroll to top of page on load
    scroll_to_top();

    html!(
        <div class={stylesheet}>
            <div class={"funder"}>
                <NavBar />
                <header>
                    <h1> {"Funder"} </h1>
                    <p>{"Funder is a Solana on-chain program."}</p>
                </header>
                <ContactFooter />
            </div>
        </div>
    )
}
