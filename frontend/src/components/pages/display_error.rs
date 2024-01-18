//! Webpage that displays error messages.

use stylist::{yew::styled_component, Style};
use yew::prelude::*;

use crate::components::{
    pages::scroll_to_top,
    subcomponents::{contact_footer::ContactFooter, nav_bar::NavBar},
};

use crate::styles::STYLESHEET;

/// The error message to display on on the page.
#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub error_message: String,
}

/// The page of the web application that displays the error messages.
#[styled_component(DisplayError)]
pub fn display_error(props: &Props) -> Html {
    let stylesheet = Style::new(STYLESHEET).expect("Failed to create style");

    // Scroll to top of page on load
    scroll_to_top();

    html!(
        <div class={stylesheet}>
            <div class={"error"}>
                <NavBar />
                <div class={"error-message"}>
                    <h1>{"An error has ocurred:"}</h1>
                    <h2>{&props.error_message}</h2>
                </div>
                <ContactFooter />
            </div>
        </div>
    )
}
