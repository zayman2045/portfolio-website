use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

#[function_component(Stargaze)]
pub fn stargaze() -> Html {
    html!(
        <div>
            <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
            <h1> {"Welcome to Stargaze"} </h1>
        </div>
    )
}