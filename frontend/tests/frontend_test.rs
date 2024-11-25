use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1, 1);
}

// On a target other then `wasm32-unknown-unknown`, the `#[test]` attribute
// will be used instead, allowing this test to run on any target.
// #[wasm_bindgen_test(unsupported = test)]
// fn pass_all_targets() {
//     assert_eq!(2, 2);
// }
