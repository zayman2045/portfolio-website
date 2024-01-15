//! Contains the footer of the application.

use yew::prelude::*;

/// Represents the footer of the application.
///
/// This functional component renders a footer with contact information.
/// It includes an email address and a GitHub username, both of which are rendered as links.
#[function_component(ContactFooter)]
pub fn contact_footer() -> Html {
    html!(
        <footer>
            <p>{"Contact: "}<a href="mailto:xavierorlandogriffith@gmail.com">{"xavierorlandogriffith@gmail.com"}</a></p>
            <p>{"GitHub: "}<a href="https://github.com/zayman2045">{"zayman2045"}</a></p>
        </footer>
    )
}