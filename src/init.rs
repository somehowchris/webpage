use std::cell::Cell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlImageElement;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn tm_modalbox();

    #[wasm_bindgen(js_namespace = window)]
    fn tm_nav_bg();

    #[wasm_bindgen(js_namespace = window)]
    fn tm_trigger_menu();

    #[wasm_bindgen(js_namespace = window)]
    fn tm_service_popup();

    #[wasm_bindgen(js_namespace = window)]
    fn tm_modalbox_news();

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

pub fn tm_stuff(){
    tm_modalbox();
    tm_nav_bg();
    tm_trigger_menu();
    tm_service_popup();
    tm_modalbox_news();
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

pub struct ImageFuture {
    image: Option<HtmlImageElement>,
    load_failed: Rc<Cell<bool>>,
}

impl ImageFuture {
    pub fn new(path: &str) -> Self {
        let image = HtmlImageElement::new().unwrap();
        image.set_src(path);
        ImageFuture {
            image: Some(image),
            load_failed: Rc::new(Cell::new(false)),
        }
    }
}

impl Future for ImageFuture {
    type Output = Result<HtmlImageElement, ()>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match &self.image {
            Some(image) if image.complete() => {
                let image = self.image.take().unwrap();
                let failed = self.load_failed.get();

                if failed {
                    Poll::Ready(Err(()))
                } else {
                    Poll::Ready(Ok(image))
                }
            }
            Some(image) => {
                let waker = cx.waker().clone();
                let on_load_closure = Closure::wrap(Box::new(move || {
                    waker.wake_by_ref();
                }) as Box<dyn FnMut()>);
                image.set_onload(Some(on_load_closure.as_ref().unchecked_ref()));
                on_load_closure.forget();

                let waker = cx.waker().clone();
                let failed_flag = self.load_failed.clone();
                let on_error_closure = Closure::wrap(Box::new(move || {
                    failed_flag.set(true);
                    waker.wake_by_ref();
                }) as Box<dyn FnMut()>);
                image.set_onerror(Some(on_error_closure.as_ref().unchecked_ref()));
                on_error_closure.forget();

                Poll::Pending
            }
            _ => Poll::Ready(Err(())),
        }
    }
}

pub async fn has_webp_support() -> bool {
    let img = ImageFuture::new(
        "data:image/webp;base64,UklGRjIAAABXRUJQVlA4ICYAAACyAgCdASoCAAEALmk0mk0iIiIiIgBoSygABc6zbAAA/v56QAAAAA==",
    );

    img.await.is_ok()
}


// TODO check if supports webp
pub fn build_webp_url(path: &str, default_extension: &str) -> String {
    let has_support = Arc::new(Mutex::new(false));
    let mutex = Arc::clone(&has_support);
    spawn_local(async move {
        if has_webp_support().await {
            let mut state = mutex.lock().unwrap();
            *state = true;
        }
    });

    if/*  *has_support.lock().unwrap() */ true {
        let mut url = String::with_capacity(path.len() + 1 + 4);

        url.push_str(path);
        url.push_str(".webp");

        url
    } else {
        let mut url = String::with_capacity(path.len() + 1 + default_extension.len());

        url.push_str(path);
        url.push('.');
        url.push_str(default_extension);

        url
    }
}
