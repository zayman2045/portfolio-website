use stylist::{yew::styled_component, Style};
use yew::prelude::*;

use crate::components::molecules::link_button::LinkButton;
use crate::router::Route;

const STYLE_FILE: &str = include_str!("stylesheets/stargaze.css");

#[styled_component(Stargaze)]
pub fn stargaze() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();

    html!(
        <div class={stylesheet}>
            <LinkButton route={Route::Home} label={"Home".to_string()} kind={"button".to_string()} />
            <h1> {"Welcome to Stargaze"} </h1>

            // Should only render buttons if the user is not logged in
            <h2>{"Begin Your Journey"}</h2>
            <div class={"button-container"}>
                <LinkButton route={Route::Login} label={"Log In".to_string()} kind={"button".to_string()} />
                <LinkButton route={Route::Signup} label={"Sign Up".to_string()} kind={"button".to_string()} />
            </div>
        </div>
    )
}