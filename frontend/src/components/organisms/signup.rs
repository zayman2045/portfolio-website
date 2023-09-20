use yew::prelude::*;

use crate::{
    components::{atoms::text_input::TextInput, molecules::link_button::LinkButton},
    router::Route,
};

#[function_component(Signup)]
pub fn signup() -> Html {
    html!(
        <div>
            <LinkButton route={Route::Home} label={"Home".to_string()} kind={"button".to_string()} />

            <h1>{"Sign Up"}</h1>
            <form>
                <label for="username">{"Username:"}</label>
                <TextInput name={"username".to_string()} />

                <label for="password">{"Password:"}</label>
                <TextInput name={"password".to_string()} />

                <label for="password">{"Re-enter Password:"}</label>
                <TextInput name={"password".to_string()} />

                <button type="submit">{"Submit"}</button>
            </form>
        </div>
    )
}
