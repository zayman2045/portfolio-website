use stylist::{yew::styled_component, Style};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::{
    components::molecules::link_button::LinkButton,
    router::Route,
    stores::auth_store::AuthStore,
};

const STYLE_FILE: &str = include_str!("stylesheets/signup.css");

#[styled_component(Signup)]
pub fn signup() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();

    // Create shared state to hold authentication information from text inputs
    let (auth_store, auth_dispatch) = use_store::<AuthStore>();
    
    // Onchange of text inputs, store the text in state
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
                <input type="text" id="password" placeholder="Password" required=true/>

                <label for="password">{"Confirm Password:"}</label>
                <input type="text" id="confirm_password" placeholder="Password" required=true/>

                <button type="submit">{"Submit"}</button>
            </form>

            <p>{"Username: "}{auth_store.username.clone()}</p>
        </div>
    )
}
