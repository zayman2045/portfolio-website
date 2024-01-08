use stylist::{yew::styled_component, Style};
use yew::prelude::*;
use yew_router::prelude::Link;

use crate::{components::{subcomponents::{nav_bar::NavBar, contact_footer::ContactFooter}, pages::scroll_to_top}, router::Route};

const STYLE_FILE: &str = include_str!("stylesheets/styles.css");

#[styled_component(AboutProjects)]
pub fn stargaze() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();

    // Scroll to top of page on load
    scroll_to_top();

    html!(
        <div class={stylesheet}>
            <div class={"about-projects"}>
                <NavBar />
                <div class={"content-container"}>
                    <Link<Route> to={Route::Stargaze}>
                        <img src={"img/stargaze-portrait.jpeg"} />
                    </Link<Route>>
                    <Link<Route> to={Route::Funder}>
                        <img src={"img/funder-portrait.jpeg"} />
                    </Link<Route>>
                    <Link<Route> to={Route::Ello}>
                        <img src={"img/ello-portrait.jpeg"} />
                    </Link<Route>>
                </div>
                <ContactFooter />
            </div>
        </div>
    )
}
