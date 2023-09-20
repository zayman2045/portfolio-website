use yew::prelude::*;

use crate::components::molecules::link_button::LinkButton;
use crate::router::Route;

#[function_component(Funder)]
pub fn funder() -> Html {
    html!(
        <div>
            <LinkButton route={Route::Home} label={"Home".to_string()} kind={"button".to_string()} />
            <h1> {"Bring the Thunder to Funder"} </h1>

            // Should only render if the user is not logged in
            <h2>{"Begin Your Journey"}</h2>
            <LinkButton route={Route::Login} label={"Log In".to_string()} kind={"button".to_string()} />
            <LinkButton route={Route::Signup} label={"Sign Up".to_string()} kind={"button".to_string()} />
        </div>
    )
}
