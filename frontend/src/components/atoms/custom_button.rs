use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: String,
    pub kind: String,
}

#[function_component(CustomButton)]
pub fn custom_button(props: &Props) -> Html {
    html!(
        <button type={props.kind.clone()}>{props.label.clone()}</button>
    )
}
