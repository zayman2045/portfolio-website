//! Webpage for logging in to the website.

use stylist::{yew::styled_component, Style};
use yew::prelude::*;

use crate::components::{subcomponents::{nav_bar::NavBar, contact_footer::ContactFooter}, pages::scroll_to_top};

const STYLE_FILE: &str = include_str!("stylesheets/styles.css");

/// Represents the page of the web application that allows users to log in.
#[styled_component(Login)]
pub fn login() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();

    // Scroll to top of page on load
    scroll_to_top();

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
