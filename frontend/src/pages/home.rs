use yew::prelude::*;

// Displays the website homepage
#[function_component(Home)]
pub fn home() -> Html {
    html!(
        <>
            <h1>{"This is the Homepage!"}</h1>
        </>
    )
}
