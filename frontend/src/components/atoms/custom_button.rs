use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub content: String,
    pub kind: String,
}

#[function_component(CustomButton)]
pub fn custom_button(props: &Props) -> Html {
    html!(
        <button type={props.kind.clone()}>{props.content.clone()}</button>
    )
}
