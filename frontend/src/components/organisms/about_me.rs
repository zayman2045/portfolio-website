use stylist::{yew::styled_component, Style};
use yew::prelude::*;

use crate::components::molecules::nav_bar::NavBar;

const STYLE_FILE: &str = include_str!("stylesheets/styles.css");

#[styled_component(AboutMe)]
pub fn stargaze() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();

    html!(
        <div class={stylesheet}>
            <div class={"about-me"}>
                <NavBar />
                <h1> {"Xavier Griffith"} </h1>
                <p>{"I am a software developer proficient in the Rust programming language."}</p>
            </div>
        </div>
    )
}