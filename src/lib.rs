use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[wasm_bindgen]
pub extern "C" fn greet() {
    web_sys::console::log_1(&"Hello from Rust and WebAssembly!".into());
}
