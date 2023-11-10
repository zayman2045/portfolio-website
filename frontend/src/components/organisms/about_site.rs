use stylist::{yew::styled_component, Style};
use yew::prelude::*;

use crate::components::molecules::{contact_footer::ContactFooter, nav_bar::NavBar};

const STYLE_FILE: &str = include_str!("stylesheets/styles.css");

#[styled_component(AboutSite)]
pub fn stargaze() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();

    // Scroll to top of page on load
    crate::scroll_to_top();

    html!(
        <div class={stylesheet}>
            <div class={"about-site"}>
                <NavBar />
                <div class={"crate-container"}>
                    <img src={"img/yew-logo.jpeg"} alt={"Yew Logo"} />
                    <div class={"content-container"}>
                        <h2>{"Yew"} </h2>
                        <p>{"The front-end of this website was built using Yew, a modern Rust framework for creating multi-threaded web apps with WebAssembly. Yew is inspired by modern JavaScript frameworks such as React and Vue, which use a component-based architecture that allows for the reuse of UI elements, and a lightweight Virtual DOM to improve performance."}</p>
                    </div>
                </div>
                <div class={"crate-container"}>
                    <img src={"img/tokio-logo.jpeg"} alt={"Tokio Logo"} />
                    <div class={"content-container"}>
                        <h2>{"Axum"} </h2>
                        <p>{"The back-end of this site is coded using Axum. Axum is a web application framework that focuses on ergonomics and modularity."}</p>
                    </div>
                </div>
                <div class={"crate-container"}>
                    <img src={"img/seaorm-logo.jpeg"} alt={"SeaORM Logo"} />
                    <div class={"content-container"}>
                        <h2>{"SeaORM"} </h2>
                        <p>{"SeaORM is a relational ORM to help you build light-weight and scalable web services in Rust."}</p>
                    </div>
                </div>
                <div class={"crate-container"}>
                    <img src={"img/postgres-logo.jpeg"} alt={"Postgres Logo"} />
                    <div class={"content-container"}>
                        <h2>{"Postgres"} </h2>
                        <p>{"PostgreSQL is a powerful, open source object-relational database system."}</p>
                    </div>
                </div>
                <div class={"crate-container"}>
                    <img src={"img/docker-logo.jpeg"} alt={"Docker Logo"} />
                    <div class={"content-container"}>
                        <h2>{"Docker"} </h2>
                        <p>{"Docker is a set of platform as a service products that use OS-level virtualization to deliver software in packages called containers."}</p>
                    </div>
                </div>
                <ContactFooter />
            </div>
        </div>
    )
}
