use stylist::{yew::styled_component, Style};
use yew::prelude::*;

use crate::components::molecules::nav_bar::NavBar;

const STYLE_FILE: &str = include_str!("stylesheets/styles.css");

#[styled_component(AboutProjects)]
pub fn stargaze() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();

    html!(
        <div class={stylesheet}>
            <div class={"about-projects"}>
                <NavBar />
                <h1> {"Explore My Projects"} </h1>
                <div class={"content-container"}>
                    <img src={"img/stargaze-portrait.jpeg"} />
                    <img src={"img/funder-portrait.jpeg"} />
                    <img src={"img/ello-portrait.jpeg"} />
                </div>
            </div>
        </div>
    )
}
