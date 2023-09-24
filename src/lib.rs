extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
// JavaScript によって提供される alert 関数の定義
#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// JavaScript が呼び出せる Rust 関数
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

pub fn output_log(name: &str) {
    log(&format!("Hello {}!!!", name))
}
