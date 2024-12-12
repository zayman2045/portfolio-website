#[cfg(test)]
pub mod tests {
    use js_sys::Promise;
    use std::fmt;
    use wasm_bindgen_futures::JsFuture;
    use wasm_bindgen_test::*;
    use web_sys::js_sys::{self};
    use web_sys::wasm_bindgen::closure::Closure;
    use web_sys::wasm_bindgen::{JsCast, JsValue};
    use yew::prelude::*;

    // Custom error type
    #[derive(Debug)]
    struct TestError(String);

    // Implement Display for better error handling
    impl fmt::Display for TestError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    // Implement std::error::Error
    impl std::error::Error for TestError {}

    impl From<JsValue> for TestError {
        fn from(value: JsValue) -> Self {
            TestError(format!("JavaScript error: {:?}", value))
        }
    }

    impl From<&str> for TestError {
        fn from(value: &str) -> Self {
            TestError(value.to_string())
        }
    }

    // Configure tests to run in browser
    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn _test_minimal_component() -> Result<(), TestError> {
        // Define simple component
        #[function_component]
        fn SimpleComponent() -> Html {
            html! {
                <div>{"Hello Test"}</div>
            }
        }

        // Create container element
        let window = web_sys::window().ok_or("No global window found")?;
        let document = window.document().ok_or("No document found on window")?;
        let container = document.create_element("div")?;

        // Append container to body
        let body = document.body().ok_or("document should have a body")?;
        body.append_child(&container)?;

        // Render component
        yew::Renderer::<SimpleComponent>::with_root(container.clone()).render();

        // Wait for next animation frame
        let callback = Closure::once(|| {});
        let handle = window.request_animation_frame(callback.as_ref().unchecked_ref())?;

        // Convert to Promise and await
        let promise = Promise::resolve(&JsValue::from(handle));
        let _ = JsFuture::from(promise).await?;

        // Assert content
        assert_eq!(container.inner_html(), "<div>Hello Test</div>");

        // Cleanup
        container.remove();

        Ok(())
    }
}
