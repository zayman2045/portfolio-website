use gloo_net::http::Request;
use web_sys::console;
use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::*;

// Create yew routes that will be requested by the browser
#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/hello-server")]
    HelloServer,
}

// Dispatch routes and render functional components
fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
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

// Displays the website homepage
#[function_component(Home)]
fn home_page() -> Html {
    html!(
        <>
            <h1>{"This is the Homepage!"}</h1>
        </>
    )
}

// Sends a request to the server and displays the response
#[function_component(HelloServer)]
fn hello_server() -> Html {
    // Create state
    let data = use_state(|| None);

    {
        let data = data.clone();

        // Perform side effect after render
        use_effect(move || {
            if data.is_none() {
                spawn_local(async move {
                    // Make a request to the Axum api using proxy
                    let response = Request::get("/api/hello-server").send().await;

                    let result = match response {
                        Ok(response) => {
                            if response.ok() {
                                response.text().await.map_err(|err| err.to_string())
                            } else {
                                Err(format!(
                                    "Error fetching data {} ({})",
                                    response.status(),
                                    response.status_text()
                                ))
                            }
                        }
                        Err(err) => Err(err.to_string()),
                    };

                    // Log the api response to the console
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
