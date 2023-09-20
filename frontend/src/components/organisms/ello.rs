use yew::prelude::*;
use yew_router::prelude::*;

use crate::{router::Route, components::molecules::auth_buttons::AuthButtons};

#[function_component(Ello)]
pub fn ello() -> Html {
    html!(
        <div>
            <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
            <h1> {"Say Hello to Ello!"} </h1>

            // Should only render if the user is not logged in
            <AuthButtons />
        </div>
    )
}
