use std::cell::Cell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
use std::task::Context;
use std::task::Poll;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlImageElement;

pub struct ImageFuture {
    image: Option<HtmlImageElement>,
    load_failed: Rc<Cell<Option<bool>>>,
}

impl ImageFuture {
    pub fn new(path: &str) -> Self {
        let image = HtmlImageElement::new().unwrap();
        image.set_src(path);

        let did_fail = Rc::new(Cell::new(None));

        let on_load_did_fail = did_fail.clone();
        let on_load_closure = Closure::wrap(Box::new(move || {
            on_load_did_fail.set(Some(false));
        }) as Box<dyn FnMut()>);
        image.set_onload(Some(on_load_closure.as_ref().unchecked_ref()));
        on_load_closure.forget();

        let on_error_did_fail = did_fail.clone();
        let on_error_closure = Closure::wrap(Box::new(move || {
            on_error_did_fail.set(Some(true));
        }) as Box<dyn FnMut()>);
        image.set_onerror(Some(on_error_closure.as_ref().unchecked_ref()));
        on_error_closure.forget();

        ImageFuture {
            image: Some(image),
            load_failed: did_fail,
        }
    }
}

impl Future for ImageFuture {
    type Output = Result<HtmlImageElement, ()>;

    fn poll(mut self: Pin<&mut Self>, _ctx: &mut Context<'_>) -> Poll<Self::Output> {
        match &self.image {
            Some(image) if image.complete() => {
                if let Some(did_fail) = self.load_failed.get() {
                    if did_fail {
                        log::info!("Fail");
                        Poll::Ready(Err(()))
                    } else {
                        Poll::Ready(Ok(self.image.take().unwrap()))
                    }
                } else {
                    Poll::Pending
                }
            }
            Some(_image) => Poll::Pending,
            _ => {
                log::info!("Unkown result");
                Poll::Ready(Err(()))
            }
        }
    }
}

pub async fn has_webp_support() -> bool {
    let img = ImageFuture::new(
        "data:image/webp;base64,UklGRjIAAABXRUJQVlA4ICYAAACyAgCdASoCAAEALmk0mk0iIiIiIgBoSygABc6zbAAA/v56QAAAAA==",
    );

    let result = img.await.is_ok();

    log::info!("Result of image load {}", result);

    result
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

    if
    /*  *has_support.lock().unwrap() */
    true {
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
