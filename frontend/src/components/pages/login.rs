//! Webpage for logging in to the website.

use reqwasm::http::Request;
use serde_json::json;
use stylist::{yew::styled_component, Style};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::hooks::use_navigator;
use yewdux::dispatch::Dispatch;

use crate::{
    components::{
        pages::scroll_to_top,
        subcomponents::{contact_footer::ContactFooter, nav_bar::NavBar},
        types::UserResponse,
    },
    router::Route,
    stores::{auth_store::AuthStore, user_store::UserStore},
};

const STYLE_FILE: &str = include_str!("stylesheets/styles.css");

/// The properties of the Login component.
#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub message: Option<String>,
}

/// The page of the web application that allows users to log in.
#[styled_component(Login)]
pub fn login(props: &Props) -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();

    // Scroll to top of page on load
    scroll_to_top();

    // Use Yewdux store to hold authentication information from text inputs
    let auth_dispatch = Dispatch::<AuthStore>::new();

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

    // Use navigator to redirect the user after a successful log in
    let navigator = use_navigator().unwrap();

    // Handler for log in form submission
    let onsubmit = auth_dispatch.reduce_mut_callback_with(move |auth_store, event: SubmitEvent| {
        event.prevent_default();

        let auth_store = auth_store.clone();
        let navigator = navigator.clone();

        // Spawn a new thread
        wasm_bindgen_futures::spawn_local(async move {
            let user = json!({
                "username": auth_store.username,
                "password": auth_store.password,
            })
            .to_string();

            // Send a POST request to the backend API to log in user
            let response = Request::post("/api/login")
                .body(user)
                .header("content-type", "application/json")
                .send()
                .await
                .unwrap();

            match response.status() {
                // User logged in successfully
                200 => {
                    let user: UserResponse = response.json().await.unwrap();
                    let user_dispatch = Dispatch::<UserStore>::new();

                    // Update the UserStore
                    user_dispatch.reduce_mut(|user_store| {
                        user_store.username = user.username.clone();
                        user_store.token = user.token.clone();
                        user_store.id = user.id.clone();
                    });

                    // Redirect the user to their mission page
                    let username = user_dispatch.get().username.as_ref().unwrap().clone();
                    navigator.push(&Route::MissionsUsers { username });
                }

                // User credentials are incorrect
                404 | 401 => {
                    navigator.push(&Route::LoginInvalid);
                }
                // Error logging in user
                _ => {
                    navigator.push(&Route::LoginError);
                }
            }
        });
    });

    html!(
        <div class={stylesheet}>
            <div class={"login"}>
                <NavBar />

                <h1>{"Log In"}</h1>

                if let Some(message) = props.message.as_ref() {
                    <h2>{message}</h2>
                }

                <form {onsubmit}>
                    <label for="username">{"Username:"}</label>
                    <input type="text" id="username" placeholder="Username" required=true onchange={onchange_username}/>

                    <label for="password">{"Password:"}</label>
                    <input type="password" id="password" placeholder="Password" required=true onchange={onchange_password}/>

                    <button type="submit">{"Submit"}</button>
                </form>
                <ContactFooter />
            </div>
        </div>
    )
}
