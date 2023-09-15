use stylist::{yew::styled_component, Style};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

const STYLE_FILE: &str = include_str!("stylesheets/home.css");

// Displays the website homepage
#[styled_component(Home)]
pub fn home() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();

    html!(
        <div class={stylesheet}>
            <header>
                <h1>{"Xavier Griffith"}</h1>
                <p>{"Specializing in Rust Development"}</p>
            </header>

            <section>
                <h2>{"About Me"}</h2>
                <p>{"I am a software developer proficient in the Rust programming language."}</p>
            </section>

            <section>
                <h2>{"About This Site"}</h2>
                <p>{"(Description of the site's architecture. Yew + Axum)"}</p>
                <p>{"(Short description of each projects. More detailed descriptions in an info section within each project.)"}</p>
            </section>

            <section class="projects">
                <h2>{"Explore My Projects"}</h2>
                <ul>
                    <li><Link<Route> to={Route::Stargaze}>{ "Stargaze" }</Link<Route>></li>
                    <li><Link<Route> to={Route::Funder}>{ "Funder" }</Link<Route>></li>
                    <li><Link<Route> to={Route::Ello}>{ "Ello" }</Link<Route>></li>
                </ul>
            </section>

            <footer>
                <p>{"Contact: "}<a href="mailto:xavierorlandogriffith@gmail.com">{"xavierorlandogriffith@gmail.com"}</a></p>
                <p>{"GitHub: "}<a href="https://github.com/zayman2045">{"zayman2045"}</a></p>
            </footer>
        </div>
    )
}
