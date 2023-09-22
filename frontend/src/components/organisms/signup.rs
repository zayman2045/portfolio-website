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
    let onchange_username = {
        let dispatch = auth_dispatch.clone();

        Callback::from(move |event: Event| {
            let username = event.target_unchecked_into::<HtmlInputElement>().value();

            let username = if username.is_empty() {
                None
            } else {
                Some(username)
            };

            dispatch.reduce_mut(|store| store.username = username);
        })
    };

    // Store password when <input/> onchange event occurs
    let onchange_password = {
        let dispatch = auth_dispatch.clone();

        Callback::from(move |event: Event| {
            let password = event.target_unchecked_into::<HtmlInputElement>().value();

            let password = if password.is_empty() {
                None
            } else {
                Some(password)
            };

            dispatch.reduce_mut(|store| store.password = password);
        })
    };

    let onchange_confirm_password = {
        let dispatch = auth_dispatch.clone();

        Callback::from(move |event: Event| {
            let confirm_password = event.target_unchecked_into::<HtmlInputElement>().value();

            let confirm_password = if confirm_password.is_empty() {
                None
            } else {
                Some(confirm_password)
            };

            dispatch.reduce_mut(|store| store.confirm_password = confirm_password);
        })
    };

    // On form submission send a request to the API verifying the username does not exist and then creating the user
    //let on_submit =

    html!(
        <div class={stylesheet}>
            <LinkButton route={Route::Home} label={"Home".to_string()} kind={"button".to_string()} />

            <h1>{"Sign Up"}</h1>
            <form>
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
        </div>
    )
}
