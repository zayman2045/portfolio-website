use yew::prelude::*;
use yew_router::prelude::*;

// Create yew router
#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
}

// Dispatch routes
fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Hello Frontend!!!" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
