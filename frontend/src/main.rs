use gloo_net::http::Request;
use web_sys::console;
use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::*;

// Create yew router
#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/hello")]
    HelloServer,
}

// Dispatch routes
fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Hello Frontend!!!" }</h1> },
        Route::HelloServer => html! { <HelloServer /> },
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

#[function_component(HelloServer)]
fn hello_server() -> Html {
    // Create state
    let data = use_state(|| None);
    {
        let data = data.clone();

        // Perform side effect after render cycle
        use_effect(move || {
            if data.is_none() {
                spawn_local(async move {
                    // Make a request to the api
                    let resp = Request::get("/api/hello-server").send().await;
                    let result = match resp {
                        Ok(resp) => {
                            if resp.ok() {
                                resp.text().await.map_err(|err| err.to_string())
                            } else {
                                Err(format!(
                                    "Error fetching data {} ({})",
                                    resp.status(),
                                    resp.status_text()
                                ))
                            }
                        }
                        Err(err) => Err(err.to_string()),
                    };
                    match &result {
                        Ok(data) => {
                            console::log_1(&data.into());
                        }
                        Err(err) => {
                            console::log_1(&err.into());
                        }
                    }
                    data.set(Some(result));
                });
            }

            || {}
        });
    }

    match data.as_ref() {
        None => {
            html! {
                <div>{"No server response"}</div>
            }
        }
        Some(Ok(data)) => {
            html! {
                <div>{"Got server response: "}{data}</div>

            }
        }
        Some(Err(err)) => {
            html! {
                <div>{"Error requesting data from server: "}{err}</div>
            }
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
