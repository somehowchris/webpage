use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(js_namespace = window)]
    fn tm_modalbox_portfolio();

    #[wasm_bindgen(js_namespace = window)]
    fn progress_by_frenify();

    #[wasm_bindgen(js_namespace = window)]
    fn tm_mycounter();

    #[wasm_bindgen(js_namespace = window)]
    fn tm_projects();

    #[wasm_bindgen(js_namespace = window)]
    fn tm_portfolio();

    #[wasm_bindgen(js_namespace = window)]
    fn tm_cursor();

    #[wasm_bindgen(js_namespace = window)]
    fn tm_imgtosvg();

    #[wasm_bindgen(js_namespace = window)]
    fn tm_popup();

    #[wasm_bindgen(js_namespace = window)]
    fn tm_data_images();

    #[wasm_bindgen(js_namespace = window)]
    fn tm_contact_form();

    #[wasm_bindgen(js_namespace = window)]
    fn tm_input_padding();

    #[wasm_bindgen(js_namespace = window)]
    fn tm_totop();

    #[wasm_bindgen(js_namespace = window)]
    fn tm_down();

    #[wasm_bindgen(js_namespace = window)]
    fn tm_my_load();
}

pub fn tm_stuff() {
    tm_modalbox_portfolio();
    progress_by_frenify();
    tm_mycounter();
    tm_projects();
    tm_portfolio();
    tm_cursor();
    tm_imgtosvg();
    tm_popup();
    tm_data_images();
    tm_contact_form();
    tm_input_padding();
    tm_totop();
    tm_down();
    tm_my_load();
}
