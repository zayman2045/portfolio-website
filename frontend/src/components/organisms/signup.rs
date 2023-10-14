use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::json;
use stylist::{yew::styled_component, Style};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::{
    components::molecules::nav_bar::NavBar,
    router::Route,
    stores::{auth_store::AuthStore, user_store::UserStore},
};

const STYLE_FILE: &str = include_str!("stylesheets/styles.css");

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

    // Store username when onchange event occurs to the username input field
    let onchange_username = auth_dispatch.reduce_mut_callback_with(|auth_store, event: Event| {
        let username = event.target_unchecked_into::<HtmlInputElement>().value();
        auth_store.username = if username.is_empty() {
            None
        } else {
            Some(username)
        }
    });

    // Store password when onchange event occurs to the password input field
    let onchange_password = auth_dispatch.reduce_mut_callback_with(|auth_store, event: Event| {
        let password = event.target_unchecked_into::<HtmlInputElement>().value();
        auth_store.password = if password.is_empty() {
            None
        } else {
            Some(password)
        }
    });

    // Store confirmed password when onchange event occurs to the confirmed password input field
    let onchange_confirmed_password =
        auth_dispatch.reduce_mut_callback_with(|auth_store, event: Event| {
            let confirmed_password = event.target_unchecked_into::<HtmlInputElement>().value();
            auth_store.confirmed_password = if confirmed_password.is_empty() {
                None
            } else {
                Some(confirmed_password)
            }
        });

    // Use navigator to redirect after a successful sign up
    let navigator = use_navigator().unwrap();

    // Handler for sign up form submission
    let onsubmit = auth_dispatch.reduce_mut_callback_with(move |auth_store, event: SubmitEvent| {
        event.prevent_default();
        auth_store.message = None;
        // Verify the passwords match before making a POST request
        if auth_store.password != auth_store.confirmed_password {
            auth_store.message = Some("Passwords Do Not Match".to_owned());
        } else {
            // Make a POST request to the backend to create a new user
            let auth_store = auth_store.clone();
            let navigator = navigator.clone();
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

                        // TODO: Redirect to the page the user came from
                        navigator.push(&Route::Home);
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
            <div class={"login"}>
                <NavBar />

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
        </div>
    )
}
