//! Webpage for displaying projects.

use stylist::{yew::styled_component, Style};
use yew::prelude::*;


use crate::components::{subcomponents::{nav_bar::NavBar, contact_footer::ContactFooter}, pages::scroll_to_top};

const STYLE_FILE: &str = include_str!("stylesheets/styles.css");

/// Represents the page of the web application that describes the projects.
#[styled_component(AboutProjects)]
pub fn about_projects() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();

    // Scroll to top of page on load 
    scroll_to_top();

    html!(
        <div class={stylesheet}>
            <div class={"about-projects"}>
                <NavBar />
                <div class={"content-container"}>
                    // <Link<Route> to={Route::Stargaze}>
                    //     <img src={"img/stargaze-portrait.jpeg"} />
                    // </Link<Route>>
                    // <Link<Route> to={Route::Funder}>
                    //     <img src={"img/funder-portrait.jpeg"} />
                    // </Link<Route>>
                    // <Link<Route> to={Route::Ello}>
                    //     <img src={"img/ello-portrait.jpeg"} />
                    // </Link<Route>>
                    <a href={"https://github.com/zayman2045/stargaze"}>
                        <img src={"img/stargaze-portrait.jpeg"} />
                    </a>
                    <a href={"https://github.com/zayman2045/funder"}>
                        <img src={"img/funder-portrait.jpeg"} />
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
