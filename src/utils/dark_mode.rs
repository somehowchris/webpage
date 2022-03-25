use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

fn check_dark_mode_settings() {
    if let Some(window) = web_sys::window() {
        let query_result = window.match_media("(prefers-color-scheme: dark)");
        if let Ok(Some(mql)) = query_result {
            let body = window
                .document()
                .unwrap()
                .body()
                .expect("document should have a body");
            let classes = body.class_list();

            let array = js_sys::Array::new();

            array.push(&wasm_bindgen::JsValue::from_str("dark"));
            if mql.matches() {
                if !classes.contains("dark") {
                    classes.add(&array).unwrap()
                }
            } else if classes.contains("dark") {
                classes.remove(&array).unwrap();
            }
        }
    }
}

pub fn install_dark_mode_media_query() {
    check_dark_mode_settings();
    if let Some(window) = web_sys::window() {
        let query_result = window.match_media("(prefers-color-scheme: dark)");
        if let Ok(Some(mql)) = query_result {
            let f = Closure::wrap(Box::new(check_dark_mode_settings) as Box<dyn FnMut()>);
            mql.set_onchange(Some(f.as_ref().unchecked_ref()));
            f.forget();
        }
    }
}
