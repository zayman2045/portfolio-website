use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::atoms::custom_button::CustomButton;
use crate::router::Route;

// Links to login and signup pages
#[function_component(AuthButtons)]
pub fn auth_buttons() -> Html {
    html!(
        <div>
            <Link<Route> to={Route::Login}>
                <CustomButton label={"Log In".to_string()} kind={"button".to_string()}/>
            </Link<Route>>
            <Link<Route> to={Route::Signup}>
                <CustomButton label={"Sign Up".to_string()} kind={"button".to_string()}/>
            </Link<Route>>
        </div>
    )
}
