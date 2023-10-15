use stylist::{yew::styled_component, Style};
use yew::prelude::*;

use crate::components::molecules::nav_bar::NavBar;

const STYLE_FILE: &str = include_str!("stylesheets/styles.css");

#[styled_component(AboutSite)]
pub fn stargaze() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();

    html!(
        <div class={stylesheet}>
            <div class={"about-site"}>
                <NavBar />
                <h1> {"About This Website"} </h1>
                <p>{"..."}</p>
            </div>
        </div>
    )
}