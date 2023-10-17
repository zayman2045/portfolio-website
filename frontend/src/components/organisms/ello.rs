use stylist::{yew::styled_component, Style};
use yew::prelude::*;

use crate::components::molecules::contact_footer::ContactFooter;
use crate::components::molecules::link_button::LinkButton;
use crate::components::molecules::nav_bar::NavBar;
use crate::router::Route;

const STYLE_FILE: &str = include_str!("stylesheets/styles.css");

// Render the Ello project homepage
#[styled_component(Ello)]
pub fn ello() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();

    html!(
        <div class={stylesheet}>
            <div class={"ello"}>
                <NavBar />
                <header>
                    <h1> {"Ello"} </h1>
                    <p>{"Ello is a ..."}</p>
                </header>
                // Should only render if the user is not authenticated
                <h2>{"Begin Your Journey"}</h2>
                <div class={"button-container"}>
                    <LinkButton route={Route::Login} label={"Log In".to_string()} kind={"button".to_string()} />
                    <LinkButton route={Route::Signup} label={"Sign Up".to_string()} kind={"button".to_string()} />
                </div>
                <ContactFooter />
            </div>
        </div>
    )
}
