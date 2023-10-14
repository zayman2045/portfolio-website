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
                <LinkButton route={Route::Home} label={"About Me".to_string()} kind={"button".to_string()} />
                <LinkButton route={Route::Home} label={"About This Site".to_string()} kind={"button".to_string()} />
                <LinkButton route={Route::Home} label={"Projects".to_string()} kind={"button".to_string()} />
                </div>
            </div>
        </nav>
    )
}
