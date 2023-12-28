use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn checking_wasm() -> String {
    "Hello from Rust!".to_string()
}
