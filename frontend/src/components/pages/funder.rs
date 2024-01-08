use stylist::{yew::styled_component, Style};
use yew::prelude::*;

use crate::components::pages::scroll_to_top;
use crate::components::subcomponents::contact_footer::ContactFooter;
use crate::components::subcomponents::nav_bar::NavBar;

const STYLE_FILE: &str = include_str!("stylesheets/styles.css");

// Render the Funder project homepage
#[styled_component(Funder)]
pub fn funder() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();

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
