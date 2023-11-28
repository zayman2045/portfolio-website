use yew::prelude::*;

// Footer that contains contact information
#[function_component(ContactFooter)]
pub fn link_button() -> Html {
    html!(
        <footer>
            <p>{"Contact: "}<a href="mailto:xavierorlandogriffith@gmail.com">{"xavierorlandogriffith@gmail.com"}</a></p>
            <p>{"GitHub: "}<a href="https://github.com/zayman2045">{"zayman2045"}</a></p>
        </footer>
    )
}