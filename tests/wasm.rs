extern crate wasm_bindgen;
extern crate wasm_bindgen_test;
use wasm_bindgen::*;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn test_eval(){
    liquers_wasm::register_hello();
    let result = liquers_wasm::eval("world".into(), "hello").unwrap();
    assert_eq!(result, JsValue::from_str("Hello, world!"));
}
