use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
}

#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    html!(
        if props.name == "password" {
            <input type={props.name.clone()} name={props.name.clone()} required=true />
        } else {
            <input type="text" name={props.name.clone()} required=true />
        }
    )
}
