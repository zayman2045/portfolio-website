use stylist::{yew::styled_component, Style};
use yew::prelude::*;

use crate::{
    components::molecules::link_button::LinkButton,
    router::Route,
};

const STYLE_FILE: &str = include_str!("stylesheets/login.css");

// Renders the log in page
#[styled_component(Login)]
pub fn login() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();

    html!(
        <div class={stylesheet}>
            <LinkButton route={Route::Home} label={"Home".to_string()} kind={"button".to_string()} />

            <h1>{"Log In"}</h1>
            <form>
                <label for="username">{"Username:"}</label>
                <input type="text" id="username" placeholder="Username" required=true/>

                <label for="password">{"Password:"}</label>
                <input type="password" id="password" placeholder="Password" required=true/>

                <button type="submit">{"Submit"}</button>
            </form>
        </div>
    )
}
