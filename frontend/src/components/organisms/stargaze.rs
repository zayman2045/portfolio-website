use stylist::{yew::styled_component, Style};
use yew::prelude::*;

use crate::components::molecules::contact_footer::ContactFooter;
use crate::components::molecules::nav_bar::NavBar;

const STYLE_FILE: &str = include_str!("stylesheets/styles.css");

// Render the Stargaze project homepage
#[styled_component(Stargaze)]
pub fn stargaze() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();

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
