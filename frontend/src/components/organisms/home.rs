use stylist::{yew::styled_component, Style};
use yew::prelude::*;
use yew_router::prelude::Link;


use crate::components::molecules::contact_footer::ContactFooter;
use crate::components::molecules::nav_bar::NavBar;
use crate::router::Route;

const STYLE_FILE: &str = include_str!("stylesheets/styles.css");

// Renders the website homepage
#[styled_component(Home)]
pub fn home() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();

    html!(
        <div class={stylesheet}>
            <div class={"home"}>
                <NavBar />
                <img class={"background"} src={"img/space-background.jpeg"} alt={"Space Background"} />
                <section>
                    <h2>{"About Me"}</h2>
                    <p>{"I am a software developer proficient in the Rust programming language."}</p>
                    <p>{"Learn more..."}</p>
                </section>
                <section>
                    <h2>{"About This Site"}</h2>
                    <p>{"Learn more about the technologies used to build this website."}</p>
                    <p>{"(Short description of each projects. More detailed descriptions in an info section within each project.)"}</p>
                </section>
                <section class="projects">
                    <h2>{"Explore My Projects"}</h2>
                    <ul>
                        <li>
                            <Link<Route> to={Route::Stargaze}>
                                <img src={"img/stargaze-square1.jpeg"} alt={"Stargaze Logo"} id={"stargaze-square"}/>
                            </Link<Route>>
                        </li>
                        <li>
                            <Link<Route> to={Route::Funder}>
                                <img src={"img/funder-square1.jpeg"} alt={"Funder Logo"} id={"funder-square"} />
                            </Link<Route>>
                        </li>
                        <li>
                            <Link<Route> to={Route::Ello}>
                                <img src={"img/ello-square1.jpeg"} alt={"Ello Logo"} id={"ello-square"} />
                            </Link<Route>>
                        </li>
                    </ul>
                </section>
                <ContactFooter />
            </div>
        </div>
    )
}
