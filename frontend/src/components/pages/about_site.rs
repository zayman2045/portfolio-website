use stylist::{yew::styled_component, Style};
use yew::prelude::*;

use crate::components::subcomponents::{contact_footer::ContactFooter, nav_bar::NavBar};

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
                        <p>{"The front-end of this website is built using Yew, a modern Rust framework for creating multi-threaded web apps with WebAssembly. Yew is inspired by modern JavaScript frameworks such as React and Vue, which use a component-based architecture that allows for the reuse of UI elements, and a lightweight Virtual DOM to improve performance. The framework handles the routing through client-side navigation creating a Single Page Application (SPA) that only renders the differences between routes rather than the entire webpages. It's is also responsible for making HTTP requests to the backend API, maintaining application state, and handling DOM events."}</p>
                    </div>
                </div>
                <div class={"crate-container"}>
                    <img src={"img/tokio-logo.jpeg"} alt={"Tokio Logo"} />
                    <div class={"content-container"}>
                        <h2>{"Axum"} </h2>
                        <p>{"The back-end services of this website are powered by Axum, a cutting-edge web application framework written in Rust, from the team behind the popular Tokio asynchronous runtime. Axum is designed for building highly concurrent and efficient web applications. It stands out for its ease of use, expressive routing system, and strong emphasis on modularity and composability. Leveraging the robustness of Rust, Axum provides exceptional safety and speed, making the back-end both reliable and performant." }</p>
                    </div>
                </div>
                <div class={"crate-container"}>
                    <img src={"img/seaorm-logo.jpeg"} alt={"SeaORM Logo"} />
                    <div class={"content-container"}>
                        <h2>{"SeaORM"} </h2>
                        <p>{"For database interactions on this website, I use SeaORM, an asynchronous ORM (Object-Relational Mapping) framework for Rust. SeaORM is distinguished by its intuitive and type-safe approach to database operations, simplifying the work with complex queries and relationships. Its asynchronous nature ensures efficient database communication, while its ease of use and flexibility make data management seamless and robust."}</p>
                    </div>
                </div>
                <div class={"crate-container"}>
                    <img src={"img/postgres-logo.jpeg"} alt={"Postgres Logo"} />
                    <div class={"content-container"}>
                        <h2>{"Postgres"} </h2>
                        <p>{"The data storage solution for this website is PostgreSQL, commonly known as Postgres. It's a powerful, open-source object-relational database system that excels in reliability, feature robustness, and performance."}</p>
                    </div>
                </div>
                <div class={"crate-container"}>
                    <img src={"img/docker-logo.jpeg"} alt={"Docker Logo"} />
                    <div class={"content-container"}>
                        <h2>{"Docker"} </h2>
                        <p>{"The deployment of this application is handled by Docker, a platform that allows me to package the application and its dependencies into containers. Docker containers are lightweight executable software packages that ensure the application runs seamlessly in any environment. This encapsulation facilitates consistent deployment across different systems, simplifying development, and providing flexibility to scale and update with minimal overhead."}</p>
                    </div>
                </div>
                <ContactFooter />
            </div>
        </div>
    )
}
