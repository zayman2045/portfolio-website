use stylist::{yew::styled_component, Style};
use yew::prelude::*;

use crate::components::molecules::contact_footer::ContactFooter;
use crate::components::molecules::nav_bar::NavBar;

const STYLE_FILE: &str = include_str!("stylesheets/styles.css");

// Render the Funder project homepage
#[styled_component(Funder)]
pub fn funder() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();

    // Scroll to top of page on load
    crate::scroll_to_top();

    html!(
        <div class={stylesheet}>
            <div class={"funder"}>
                <NavBar />
                <header>
                    <h1> {"Funder"} </h1>
                    <p>{"Funder is a ..."}</p>
                </header>
                <ContactFooter />
            </div>
        </div>
    )
}
