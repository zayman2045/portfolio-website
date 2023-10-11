use std::ops::Deref;

use reqwasm::http::Request;
use stylist::{yew::styled_component, Style};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::{
    components::molecules::link_button::LinkButton, router::Route, stores::auth_store::AuthStore,
};

const STYLE_FILE: &str = include_str!("stylesheets/signup.css");

// Renders the sign up page
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
        store.password = if password.is_empty() || 
        (store.confirmed_password != Some(password.clone())) {
            store.passwords_match = false;
            Some(password)
        } else {
            store.passwords_match = true;
            Some(password)
        }
    });

    // Store the confirmed password when <input/> onchange event occurs
    let onchange_confirmed_password =
        auth_dispatch.reduce_mut_callback_with(|store, event: Event| {
            let confirmed_password = event.target_unchecked_into::<HtmlInputElement>().value();

            // Verify the passwords match before saving to store
            store.confirmed_password = if confirmed_password.is_empty()
                || store.password != Some(confirmed_password.clone())
            {
                store.passwords_match = false;
                Some(confirmed_password)
            } else {
                store.passwords_match = true;
                Some(confirmed_password)
            }
        });

    // Create a state to hold the response from the backend
    let response_state = use_state(|| Some(String::new()));
    let response_state_clone = response_state.clone();

    // Handler for sign up form submission
    let onsubmit = auth_dispatch.reduce_mut_callback_with(move |store, event: SubmitEvent| {
        event.prevent_default();
        response_state_clone.set(None);

        // Display error message to the user
        if !store.passwords_match {
            response_state_clone.set(Some("Passwords do not match".to_owned()));
        } else {
            let response_state_clone = response_state_clone.clone();
            // Make a POST request to the backend to create a new user
            wasm_bindgen_futures::spawn_local(async move {
                let response = Request::get("/api/users")
                    .send()
                    .await
                    .unwrap();
                let text = response.text().await.unwrap();
                response_state_clone.set(Some(text));
            });
        }
        // Add the user to the UserStore
        // set is_authenticated to true
        // Redirect the user to the app they came from
    });

    html!(
        <div class={stylesheet}>
            <LinkButton route={Route::Home} label={"Home".to_string()} kind={"button".to_string()} />

            <h1>{"Sign Up"}</h1>

            if let Some(message) = response_state.deref() {
                <h2 style="color: red;">{message}</h2>
            }


            <form {onsubmit}>
                <label for="username">{"Username:"}</label>
                <input type="text" id="username" placeholder="Username" required=true onchange={onchange_username}/>

                <label for="password">{"Password:"}</label>
                <input type="password" id="password" placeholder="Password" required=true onchange={onchange_password}/>

                <label for="password">{"Confirm Password:"}</label>
                <input type="password" id="confirmed_password" placeholder="Password" required=true onchange={onchange_confirmed_password}/>

                <button type="submit">{"Submit"}</button>
            </form>
        </div>
    )
}
