#[cfg(test)]
pub mod tests {
    use wasm_bindgen_test::*;
    use yew::prelude::*;

    // Configure the test to run in the browser
    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_home_page_title() {
        // Define component props
        #[derive(Properties, PartialEq)]
        struct TestProps {
            title: String,
        }

        // Define component
        #[function_component]
        fn TestComponent(props: &TestProps) -> Html {
            html! {
                <div class="home">
                    <h1>{ &props.title }</h1>
                </div>
            }
        }

        // Create test title
        let expected_title = "Web3 Software Developer".to_string();
        
        // Create test component
        let props = TestProps { 
            title: expected_title.clone() 
        };
        
        // Create test root
        let root = gloo_utils::document()
            .create_element("div")
            .unwrap();
        
        // Render component
        yew::Renderer::<TestComponent>::with_root_and_props(
            root.clone(),
            props
        ).render();

        // Get rendered content
        let heading = root
            .first_child()
            .unwrap()
            .first_child()
            .unwrap();

        // Assert content
        assert_eq!(heading.text_content().unwrap(), expected_title);
    }
}