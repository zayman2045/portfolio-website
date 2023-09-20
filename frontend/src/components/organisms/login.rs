use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    components::atoms::{custom_button::CustomButton, text_input::TextInput},
    router::Route,
};

#[function_component(Login)]
pub fn login() -> Html {
    html!(
        <div>
            <Link<Route> to={Route::Home}>
                <CustomButton label={"Home".to_string()} kind={"button".to_string()}/>
            </Link<Route>>

            <h1>{"Log In"}</h1>
            <form>
                <label for="username">{"Username:"}</label>
                <TextInput name={"username".to_string()} />

                <label for="password">{"Password:"}</label>
                <TextInput name={"password".to_string()} />

                <button type="submit">{"Submit"}</button>
            </form>
        </div>
    )
}
