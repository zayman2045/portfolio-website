use stylist::{yew::styled_component, Style};
use yew::prelude::*;

use crate::{
    components::{atoms::text_input::TextInput, molecules::link_button::LinkButton},
    router::Route,
};

const STYLE_FILE: &str = include_str!("stylesheets/login.css");

// Will make API requests to get user data
#[styled_component(Login)]
pub fn login() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();

    html!(
        <div class={stylesheet}>
            <LinkButton route={Route::Home} label={"Home".to_string()} kind={"button".to_string()} />

            <h1>{"Log In"}</h1>
            <form>
                <label for="username">{"Username:"}</label>
                <TextInput name={"username".to_string()} />

                <label for="password">{"Password:"}</label>
                <TextInput name={"password".to_string()} />

                <button type="submit">{"Submit"}</button>
            </form>
        </div>
    )
}
