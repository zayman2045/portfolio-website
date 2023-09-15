use yew::prelude::*;
use stylist::{style, yew::styled_component};

// Displays the website homepage
#[styled_component(Home)]
pub fn home() -> Html {
    let stylesheet = style!(
        r#"
          h1 {
            color: pink;
          }  
        "#
    ).unwrap();

    html!(
        <div class={stylesheet}>
            <h1>{"Homepage!"}</h1>
        </div>
    )
}
