use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn progress_by_frenify();

    #[wasm_bindgen(js_namespace = window)]
    fn tm_mycounter();

    #[wasm_bindgen(js_namespace = window)]
    fn tm_cursor();

    #[wasm_bindgen(js_namespace = window)]
    fn tm_imgtosvg();

    #[wasm_bindgen(js_namespace = window)]
    fn tm_down();
}

pub fn tm_stuff() {
    /* progress_by_frenify();
    tm_mycounter();
    tm_cursor();
    tm_imgtosvg();
    tm_down(); */
}
