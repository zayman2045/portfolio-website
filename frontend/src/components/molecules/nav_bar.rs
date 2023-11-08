use yew::prelude::*;
use yew_router::prelude::Link;

use crate::router::Route;

// Navigation bar that contains a logo and links to other pages
#[function_component(NavBar)]
pub fn link_button() -> Html {
    html!(
        <nav class="navbar">
            <div class="navbar-container">
                <div class="navbar-left">
                    <Link<Route> to={Route::Home}>
                        <img class={"navbar-logo"} src={"img/xg-logo.jpeg"} alt={"XG Logo"} />
                    </Link<Route>>
                    <h1>{"Xavier Griffith"}</h1>
                    <h2>{"Specializing in Rust Development"}</h2>
                </div>
                <div class="navbar-right">
                    <Link<Route> to={Route::AboutMe}>
                        {"Meet The Developer"}
                    </Link<Route>>
                    <Link<Route> to={Route::AboutSite}>
                        {"Site Overview"}
                    </Link<Route>>
                    <Link<Route> to={Route::AboutProjects}>
                        {"My Projects"}
                    </Link<Route>>
                </div>
            </div>
        </nav>
    )
}
