use yew::prelude::*;
use yew_router::prelude::Link;

use crate::components::molecules::link_button::LinkButton;
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
                <LinkButton route={Route::AboutMe} label={"Meet The Developer".to_string()} kind={"button".to_string()} />
                <LinkButton route={Route::AboutSite} label={"Site Overview".to_string()} kind={"button".to_string()} />
                <LinkButton route={Route::AboutProjects} label={"My Projects".to_string()} kind={"button".to_string()} />
                </div>
            </div>
        </nav>
    )
}
