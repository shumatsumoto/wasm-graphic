extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
// JavaScript によって提供される alert 関数の定義
#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}
// JavaScript が呼び出せる Rust 関数
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("ddd, {}!", name));
}
