use yew::prelude::*;
use yew_router::prelude::Link;

use crate::components::molecules::link_button::LinkButton;
use crate::router::Route;

// Link that wraps a button
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
                // TODO: Conditionally render the login and sign up buttons if the user is not logged in, unless they are on a application page (Stargaze, Funder, Ello)
                // TODO: Link to the user's profile if the user is logged in
                <LinkButton route={Route::Login} label={"About Me".to_string()} kind={"button".to_string()} />
                <LinkButton route={Route::Login} label={"About This Site".to_string()} kind={"button".to_string()} />
                <LinkButton route={Route::Login} label={"Projects".to_string()} kind={"button".to_string()} />
                </div>
            </div>
        </nav>
    )
}
