use stylist::{yew::styled_component, Style};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::{
    components::molecules::link_button::LinkButton, router::Route, stores::auth_store::AuthStore,
};

const STYLE_FILE: &str = include_str!("stylesheets/signup.css");

#[styled_component(Signup)]
pub fn signup() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();

    // Create shared state (store) to hold authentication information from text inputs
    let (auth_store, auth_dispatch) = use_store::<AuthStore>();

    // Store username when <input/> onchange event occurs
    let onchange_username = auth_dispatch.reduce_mut_callback_with(|store, event: Event| {
        let username = event.target_unchecked_into::<HtmlInputElement>().value();
        store.username = if username.is_empty() {
            None
        } else {
            Some(username)
        }
    });

    // Store password when <input/> onchange event occurs
    let onchange_password = auth_dispatch.reduce_mut_callback_with(|store, event: Event| {
        let password = event.target_unchecked_into::<HtmlInputElement>().value();
        store.password = if password.is_empty() {
            None
        } else {
            Some(password)
        }
    });

    let onchange_confirm_password = auth_dispatch.reduce_mut_callback_with(|store, event: Event| {
        let confirm_password = event.target_unchecked_into::<HtmlInputElement>().value();
        store.confirm_password = if confirm_password.is_empty() {
            None
        } else {
            Some(confirm_password)
        }
    });

    let onsubmit = auth_dispatch.reduce_mut_callback_with(|store, event: SubmitEvent| {
        event.prevent_default();

        // Verify the passwords match


        // Send a request to the API to verify if the user already exists
        // Create the user in the database
        // Add the user to the UserStore and set is_logged_in to true 
        // Redirect the user to the app of their choice

        
    });

    html!(
        <div class={stylesheet}>
            <LinkButton route={Route::Home} label={"Home".to_string()} kind={"button".to_string()} />

            <h1>{"Sign Up"}</h1>
            <form {onsubmit}>
                <label for="username">{"Username:"}</label>
                <input type="text" id="username" placeholder="Username" required=true onchange={onchange_username}/>

                <label for="password">{"Password:"}</label>
                <input type="password" id="password" placeholder="Password" required=true onchange={onchange_password}/>

                <label for="password">{"Confirm Password:"}</label>
                <input type="password" id="confirm_password" placeholder="Password" required=true onchange={onchange_confirm_password}/>

                <button type="submit">{"Submit"}</button>
            </form>

            <p>{"Username: "}{auth_store.username.clone()}</p>
            <p>{"Password: "}{auth_store.password.clone()}</p>
            <p>{"Confirm Password: "}{auth_store.confirm_password.clone()}</p>
            <p>{"Passwords Match: "}{auth_store.passwords_match.clone()}</p>
        </div>
    )
}
