use std::ops::Deref;

use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::json;
use stylist::{yew::styled_component, Style};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::{
    components::molecules::link_button::LinkButton,
    router::Route,
    stores::{auth_store::AuthStore, user_store::UserStore},
};

const STYLE_FILE: &str = include_str!("stylesheets/signup.css");

#[derive(Serialize, Deserialize, Default)]
struct ResponseUser {
    username: Option<String>,
    token: Option<String>,
    message: Option<String>,
}

// Renders the sign up page
#[styled_component(Signup)]
pub fn signup() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();

    // Create shared state (store) to hold authentication information from text inputs
    let (_auth_store, auth_dispatch) = use_store::<AuthStore>();
    let (_user_store, user_dispatch) = use_store::<UserStore>();

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

    // State to hold the message from the backend
    let response_state = use_state(|| ResponseUser::default());
    let response_state_clone = response_state.clone();

    // Handler for sign up form submission
    let onsubmit = auth_dispatch.reduce_mut_callback_with(move |store, event: SubmitEvent| {
        event.prevent_default();
        response_state_clone.set(ResponseUser::default());
        let store = store.clone();

        // Display error message to the user
        if !store.passwords_match {
            response_state_clone.set(ResponseUser {
                message: Some("Passwords Do Not Match".to_owned()),
                ..Default::default()
            });
        } else {
            let response_state_clone = response_state_clone.clone();
            // Make a POST request to the backend to create a new user
            wasm_bindgen_futures::spawn_local(async move {
                let user = json!({
                    "username": store.username,
                    "password": store.password,
                })
                .to_string();

                let response = Request::post("/api/users")
                    .body(user)
                    //.body(user.to_string())
                    .header("content-type", "application/json")
                    .send()
                    .await
                    .unwrap();

                match response.status() {
                    200 => {
                        // let user: User = response.json().await.unwrap();
                        response_state_clone.set(ResponseUser {
                            message: Some("User Created".to_owned()),
                            ..Default::default()
                        });
                    }
                    409 => {
                        response_state_clone.set(ResponseUser {
                            message: Some("This Username Has Already Been Taken".to_owned()),
                            ..Default::default()
                        });
                    }
                    _ => {
                        response_state_clone.set(ResponseUser {
                            message: Some("Error Creating User".to_owned()),
                            ..Default::default()
                        });
                    }
                }
            });
        }
        // Add the username and token to the UserStore

        // Redirect the user to the app they came from
    });

    html!(
        <div class={stylesheet}>
            <LinkButton route={Route::Home} label={"Home".to_string()} kind={"button".to_string()} />

            <h1>{"Sign Up"}</h1>

            if let Some(message) = &response_state.message {
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
