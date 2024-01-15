//! Entry point for the frontend.

use frontend::App;

// Renders the App component.
fn main() {
    yew::Renderer::<App>::new().render();
}
