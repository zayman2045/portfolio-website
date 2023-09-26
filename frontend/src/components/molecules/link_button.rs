use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::atoms::custom_button::CustomButton;
use crate::router::Route;

// Takes a route, a label and a button type as properties from it's parent component
#[derive(Properties, PartialEq)]
pub struct Props {
    pub route: Route,
    pub label: String,
    pub kind: String,
}

// Link that wraps a button
#[function_component(LinkButton)]
pub fn link_button(props: &Props) -> Html {
    html!(
        <div>
            <Link<Route> to={props.route.clone()}>
                <CustomButton label={props.label.clone()} kind={props.kind.clone()}/>
            </Link<Route>>
        </div>
    )
}
