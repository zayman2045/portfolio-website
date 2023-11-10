mod components;
mod router;
mod stores;

use router::{switch, Route};
use web_sys::window;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn scroll_to_top() {
    if let Some(window) = window() {
        window.scroll_to_with_x_and_y(0.0, 0.0);
    }
}
