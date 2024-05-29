//! Webpage describing the site itself.

use stylist::{yew::styled_component, Style};
use yew::prelude::*;

use crate::components::{
    pages::scroll_to_top,
    subcomponents::{contact_footer::ContactFooter, nav_bar::NavBar},
};

use crate::styles::STYLESHEET;

/// The page of the web application that describes the site itself.
#[styled_component(AboutSite)]
pub fn about_site() -> Html {
    let stylesheet = Style::new(STYLESHEET).expect("Failed to create style");

    // Scroll to top of page on load
    scroll_to_top();

    html!(
        <div class={stylesheet}>
            <div class={"about-site"}>
                <NavBar />
                <div class={"crate-container"}>
                    <a href={"https://yew.rs/"}>
                        <img src={"img/yew-logo.jpeg"} alt={"Yew Logo"} />
                    </a>
                    <div class={"content-container"}>
                        <h2>{"Yew"} </h2>
                        <p>{"The front end of this website is built using Yew, a modern Rust framework for creating multi-threaded web applications by compiling Rust code into the WebAssembly binary instruction format which can be loaded and executed in the browser. Yew is inspired by modern JavaScript frameworks such as React and Vue, which use a component-based architecture that allows for the reuse of UI elements, and a lightweight Virtual DOM to improve performance. The framework handles the routing through client-side navigation through the yew-router crate, creating a Single Page Application (SPA) that only renders the differences between routes rather than the entire webpages. It is also responsible for making HTTP requests to the back-end API, maintaining application state using the Yewdux crate, and handling DOM events."}</p>
                    </div>
                </div>
                <div class={"crate-container"}>
                <a href={"https://github.com/tokio-rs/axum"}>
                    <img src={"img/tokio-logo.jpeg"} alt={"Tokio Logo"} />
                </a>
                    <div class={"content-container"}>
                        <h2>{"Axum"} </h2>
                        <p>{"The back-end services of this website are powered by Axum, a cutting-edge web application framework written in Rust. Developed by the team behind the popular Tokio asynchronous runtime, Axum is designed with performance and scalability in mind, able to handle thousands of concurrent connections with minimal overhead. The API layer built with Axum receives and responds to HTTP requests, converting JSON into Rust structs that are used to interact with the database and power the authentication system. It also handles the routing of requests to the appropriate handlers, sets CORS settings, and the creates JSON responses." }</p>
                    </div>
                </div>
                <div class={"crate-container"}>
                    <a href={"https://www.sea-ql.org/SeaORM/"}>
                        <img src={"img/seaorm-logo.jpeg"} alt={"SeaORM Logo"} />
                    </a>
                    <div class={"content-container"}>
                        <h2>{"SeaORM"} </h2>
                        <p>{"For the database interactions on this website, I use SeaORM, an asynchronous Object-Relational Mapping framework for Rust. SeaORM enables the execution of CRUD operations on the connected database by generating entities representing each table and allowing for their programmatic modification from within the back-end crate through the use of a powerful query builder. The database schema is converted into Rust types resulting in a type-safe interface designed to prevent runtime errors. All this without the need to write out a single line of SQL."}</p>
                    </div>
                </div>
                <div class={"crate-container"}>
                    <a href={"https://www.postgresql.org/"}>
                        <img src={"img/postgres-logo.jpeg"} alt={"Postgres Logo"} />
                    </a>
                    <div class={"content-container"}>
                        <h2>{"Postgres"} </h2>
                        <p>{"The data storage solution for this website is PostgreSQL, commonly known as Postgres, a powerful, open-source object-relational database system that excels in reliability, feature robustness, and performance. The database is set up using the Docker PostgreSQL Image, and is seeded using an init.sql file defining the schema."}</p>
                    </div>
                </div>
                <div class={"crate-container"}>
                    <a href={"https://www.docker.com/"}>
                        <img src={"img/docker-logo.jpeg"} alt={"Docker Logo"} />
                    </a>
                    <div class={"content-container"}>
                        <h2>{"Docker"} </h2>
                        <p>{"The deployment of this application is handled by Docker, a platform that allows for packaging applications and their dependencies into containers. Docker containers are lightweight executable software packages that ensure the application runs seamlessly in any environment. Both the front-end and back-end directories contain a Dockerfile with instructions for building the container, including directives for installing required software, copying data from the filesystem into the container, exposing ports and running commands to start servers. The root of the project contains a docker-compose.yaml file which handles volume management, environment variable configuration and synchronization across the front end, back end and database services."}</p>
                    </div>
                </div>
                <ContactFooter />
            </div>
        </div>
    )
}
