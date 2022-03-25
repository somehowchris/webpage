use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/init.js")]
extern "C" {
    pub fn register();
}