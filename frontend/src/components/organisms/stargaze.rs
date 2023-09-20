use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::molecules::auth_buttons::AuthButtons;
use crate::components::molecules::link_button::LinkButton;
use crate::router::Route;

#[function_component(Stargaze)]
pub fn stargaze() -> Html {
    html!(
        <div>
            <LinkButton route={Route::Home} label={"Home".to_string()} kind={"button".to_string()} />
            <h1> {"Welcome to Stargaze"} </h1>

            // Should only render buttons if the user is not logged in
            <AuthButtons />
        </div>
    )
}
