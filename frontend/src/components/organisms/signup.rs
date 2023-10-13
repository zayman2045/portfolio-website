use std::ops::Deref;

use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::json;
use stylist::{yew::styled_component, Style};
use web_sys::{console, HtmlInputElement};
use yew::prelude::*;
use yewdux::prelude::*;

use crate::{
    components::molecules::link_button::LinkButton,
    router::Route,
    stores::{
        auth_store::{self, AuthStore},
        user_store::UserStore,
    },
};

const STYLE_FILE: &str = include_str!("stylesheets/signup.css");

#[derive(Serialize, Deserialize, Default, Clone)]
struct ResponseUser {
    username: Option<String>,
    token: Option<String>,
    message: Option<String>,
}

// Renders the sign up page
#[styled_component(Signup)]
pub fn signup() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();

    // Use Yewdux store to hold authentication information from text inputs temporarily
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
        store.password =
            if password.is_empty() || (store.confirmed_password != Some(password.clone())) {
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

    // Handler for sign up form submission
    let onsubmit = auth_dispatch.reduce_mut_callback_with(move |auth_store, event: SubmitEvent| {
        event.prevent_default();
        auth_store.message = None;

        // Display error message to the user
        if !auth_store.passwords_match {
            auth_store.message = Some("Passwords Do Not Match".to_owned());
        } else {
            // Make a POST request to the backend to create a new user
            let auth_store = auth_store.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let user = json!({
                    "username": auth_store.username,
                    "password": auth_store.password,
                })
                .to_string();

                let response = Request::post("/api/users")
                    .body(user)
                    .header("content-type", "application/json")
                    .send()
                    .await
                    .unwrap();

                match response.status() {
                    // User created successfully
                    200 => {
                        let user: ResponseUser = response.json().await.unwrap();

                        let user_dispatch = Dispatch::<UserStore>::new();

                        // Update the UserStore
                        user_dispatch.reduce_mut(|user_store| {
                            user_store.username = user.username.clone();
                            user_store.token = user.token.clone();
                            user_store.message = user.message.clone();
                        });

                        // TODO: Redirect to the home page
                    }
                    // User already exists
                    409 => {
                        let auth_dispatch = Dispatch::<AuthStore>::new();

                        auth_dispatch.reduce_mut(|auth_store| {
                            auth_store.message = Some("This Username Already Exists".to_owned());
                        });
                    }
                    // Error creating user
                    _ => {
                        let auth_dispatch = Dispatch::<AuthStore>::new();

                        auth_dispatch.reduce_mut(|auth_store| {
                            auth_store.message = Some("Error Creating User".to_owned());
                        });
                    }
                }
            });
        }
    });

    html!(
        <div class={stylesheet}>
            <LinkButton route={Route::Home} label={"Home".to_string()} kind={"button".to_string()} />

            <h1>{"Sign Up"}</h1>

            if let Some(message) = &auth_store.message {
                <h2 style="color: #08f7be;">{message}</h2>
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
