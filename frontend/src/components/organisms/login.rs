use yew::prelude::*;

#[function_component(Login)]
pub fn login() -> Html {
    html!(
        <form>
            <label for="username">{"Username:"}</label>
            //<input type="text" id="username" name="username" required>

            <label for="password">{"Password:"}</label>
            //<input type="password" id="password" name="password" required>

            <button type="submit">{"Submit"}</button>
    </form>
    )
}
