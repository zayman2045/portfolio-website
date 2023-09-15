use yew::prelude::*;

#[function_component(Ello)]
pub fn ello() -> Html {
    html!(
        <div>
            <h1> {"Say Hello to Ello!"} </h1>
        </div>
    )
}
