use stylist::{yew::styled_component, Style};
use yew::prelude::*;

use crate::components::molecules::contact_footer::ContactFooter;
use crate::components::molecules::nav_bar::NavBar;

const STYLE_FILE: &str = include_str!("stylesheets/styles.css");

// Render the Ello project homepage
#[styled_component(Ello)]
pub fn ello() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();

    html!(
        <div class={stylesheet}>
            <div class={"ello"}>
                <NavBar />
                <header>
                    <h1> {"Ello"} </h1>
                    <p>{"A chatbot powered by the OpenAI GPT-3 Turbo model, designed to help users practice simple conversations in a chosen language."}</p>
                </header>
                // Should only render if the user is not authenticated
                <h2>{"Begin Your Journey"}</h2>
                <ContactFooter />
            </div>
        </div>
    )
}
