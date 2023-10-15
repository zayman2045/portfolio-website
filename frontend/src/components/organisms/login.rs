use stylist::{yew::styled_component, Style};
use yew::prelude::*;

use crate::components::molecules::{nav_bar::NavBar, contact_footer::ContactFooter};

const STYLE_FILE: &str = include_str!("stylesheets/styles.css");

// Renders the log in page
#[styled_component(Login)]
pub fn login() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();

    html!(
        <div class={stylesheet}>
            <div class={"login"}>
                <NavBar />

                <h1>{"Log In"}</h1>
                <form>
                    <label for="username">{"Username:"}</label>
                    <input type="text" id="username" placeholder="Username" required=true/>

                    <label for="password">{"Password:"}</label>
                    <input type="password" id="password" placeholder="Password" required=true/>

                    <button type="submit">{"Submit"}</button>
                </form>
                <ContactFooter />
            </div>
        </div>
    )
}
