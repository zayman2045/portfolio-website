//! Contains the navigation bar of the application.

use yew::prelude::*;
use yew_router::prelude::Link;

use crate::router::Route;

/// The navigation bar of the application.
///
/// This functional component renders a navigation bar with links to the Home, About Me, About Site, and My Projects pages.
/// Each link is represented by a `Link` component from the `yew_router` crate, with the `to` prop set to the corresponding route.
#[function_component(NavBar)]
pub fn nav_bar() -> Html {
    html!(
        <nav class="navbar">
            <div class="navbar-container">
                <div class="navbar-left">
                    <Link<Route> to={Route::Home}>
                        <img class={"navbar-logo"} src={"/img/xg-logo.jpeg"} alt={"XG Logo"} />
                    </Link<Route>>
                    <h1>{"Xavier Griffith"}</h1>
                    <h2>{"Specializing in Web3 Development"}</h2>
                </div>
                <div class="navbar-right">
                    <Link<Route> to={Route::AboutMe}>
                        {"Meet The Dev"}
                    </Link<Route>>
                    <Link<Route> to={Route::AboutSite}>
                        {"Site Overview"}
                    </Link<Route>>
                    <Link<Route> to={Route::AboutProjects}>
                        {"My Projects"}
                    </Link<Route>>
                    <Link<Route> to={Route::Missions}>
                        {"Your Missions"}
                    </Link<Route>>
                </div>
            </div>
        </nav>
    )
}
