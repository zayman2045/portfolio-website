use yew::prelude::*;

use crate::components::molecules::link_button::LinkButton;
use crate::router::Route;

#[function_component(Stargaze)]
pub fn stargaze() -> Html {
    html!(
        <div>
            <LinkButton route={Route::Home} label={"Home".to_string()} kind={"button".to_string()} />
            <h1> {"Welcome to Stargaze"} </h1>

            // Should only render buttons if the user is not logged in
            <h2>{"Begin Your Journey"}</h2>
            <LinkButton route={Route::Login} label={"Log In".to_string()} kind={"button".to_string()} />
            <LinkButton route={Route::Signup} label={"Sign Up".to_string()} kind={"button".to_string()} />
        </div>
    )
}
