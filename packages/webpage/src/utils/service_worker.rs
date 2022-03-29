use wasm_bindgen::JsValue;

pub fn register_service_worker() {
    let window = web_sys::window().expect("Thread should have window");

    wasm_bindgen_futures::spawn_local(async move {
        let registration = wasm_bindgen_futures::JsFuture::from(
            window.navigator().service_worker().register("sw.js"),
        )
        .await;

        if let Ok(ref object) = registration {
            if js_sys::Reflect::get(object, &JsValue::from("installing")).is_ok()
                && !js_sys::Reflect::get(object, &JsValue::from("installing"))
                    .unwrap()
                    .is_undefined()
            {
                log::info!("Service worker installing");
            } else if js_sys::Reflect::get(object, &JsValue::from("waiting")).is_ok()
                && !js_sys::Reflect::get(object, &JsValue::from("waiting"))
                    .unwrap()
                    .is_undefined()
            {
                log::info!("Service worker installed");
            } else if js_sys::Reflect::get(object, &JsValue::from("active")).is_ok()
                && !js_sys::Reflect::get(object, &JsValue::from("active"))
                    .unwrap()
                    .is_undefined()
            {
                log::info!("Service worker active");
            }
        } else {
            log::info!("Failed to register service worker")
        }
    });
}

