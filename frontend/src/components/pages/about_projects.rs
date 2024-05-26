//! Webpage for displaying projects.

use stylist::{yew::styled_component, Style};
use yew::prelude::*;

use crate::components::{
    pages::scroll_to_top,
    subcomponents::{contact_footer::ContactFooter, nav_bar::NavBar},
};

use crate::styles::STYLESHEET;

/// The page of the web application that describes the projects.
#[styled_component(AboutProjects)]
pub fn about_projects() -> Html {
    let stylesheet = Style::new(STYLESHEET).expect("Failed to create style");

    // Scroll to top of page on load
    scroll_to_top();

    html!(
        <div class={stylesheet}>
            <div class={"about-projects"}>
                <NavBar />
                <div class={"content-container"}>
                    <a href={"https://github.com/zayman2045/stargaze"}>
                        <img src={"img/stargaze-portrait.jpeg"} />
                    </a>
                    <a href={"https://github.com/zayman2045/novamesh"}>
                        <img src={"img/novamesh-portrait.jpeg"} />
                    </a>
                    <a href={"https://github.com/zayman2045/ello"}>
                        <img src={"img/ello-portrait.jpeg"} />
                    </a>
                </div>
                <ContactFooter />
            </div>
        </div>
    )
}
