//! Webpage for the root of the application.

use stylist::{yew::styled_component, Style};
use yew::prelude::*;
use yew_router::prelude::Link;

use crate::components::pages::scroll_to_top;
use crate::components::subcomponents::contact_footer::ContactFooter;
use crate::components::subcomponents::nav_bar::NavBar;
use crate::router::Route;
use crate::styles::STYLESHEET;


/// The home page of the web application.
#[styled_component(Home)]
pub fn home() -> Html {
    let stylesheet = Style::new(STYLESHEET).expect("Failed to create style");
    // Scroll to top of page on load
    scroll_to_top();

    html!(
        <div class={stylesheet}>
            <div class={"home"}>
                <NavBar />
                <img class={"background"} src={"img/space-background.jpeg"} alt={"Space Background"} />
                <div class={"flex-container"}>
                    <Link<Route> to={Route::AboutMe}>
                        <section>
                            <h2>{"About Me"}</h2>
                            <p>{"A short biography highlighting my technical background and interests."}</p>
                        </section>
                    </Link<Route>>
                    <Link<Route> to={Route::AboutSite}>
                        <section>
                            <h2>{"About This Site"}</h2>
                            <p>{"A brief overview of the tools and technologies used to build this website."}</p>
                        </section>
                    </Link<Route>>
                </div>
                <section class="projects">
                    <h2>{"Explore My Projects"}</h2>
                    <ul>
                        <li>
                            // <Link<Route> to={Route::Stargaze}>
                            //     <img src={"img/stargaze-square1.jpeg"} alt={"Stargaze Logo"} id={"stargaze-square"}/>
                            // </Link<Route>>
                            <a href={"https://github.com/zayman2045/stargaze"}>
                                <img src={"img/stargaze-square1.jpeg"} />
                            </a>
                        </li>
                        <li>
                            // <Link<Route> to={Route::Novamesh}>
                            //     <img src={"img/novamesh-square1.jpeg"} alt={"Novamesh Logo"} id={"novamesh-square"} />
                            // </Link<Route>>
                            <a href={"https://github.com/zayman2045/novamesh"}>
                                <img src={"img/novamesh-square1.jpeg"} />
                            </a>
                        </li>
                        <li>
                            // <Link<Route> to={Route::Ello}>
                            //     <img src={"img/ello-square1.jpeg"} alt={"Ello Logo"} id={"ello-square"} />
                            // </Link<Route>>
                            <a href={"https://github.com/zayman2045/ello"}>
                                <img src={"img/ello-square1.jpeg"} />
                            </a>
                        </li>
                    </ul>
                </section>
                <ContactFooter />
            </div>
        </div>
    )
}
