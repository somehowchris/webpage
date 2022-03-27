use yew::prelude::*;

mod components;
pub mod constants;
mod init;
pub mod utils;

pub use utils::images::build_webp_url;
use wasm_bindgen::JsObject;
use wasm_bindgen::JsValue;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

use components::{Contact, CursorPointer, Footer, Header, ScrollProgressBar, About, Welcome, Skills, Projects};

enum Msg {
    WebPSupport(bool)
}

struct Model {
    webp_support: Option<bool>
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async {
            match utils::images::has_webp_support().await {
                true => Self::Message::WebPSupport(true),
                false => Self::Message::WebPSupport(false)
            }
        });
        Self {
            webp_support: None
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::WebPSupport(value) => {
                self.webp_support = Some(value);
                true
            }
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            init::tm_stuff();
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="tm_all_wrap" data-magic-cursor="show">
                <Header />
                <Welcome webp_support={self.webp_support} />
                <About webp_support={self.webp_support} />
                <Skills />
                <Projects webp_support={self.webp_support} />
                <Contact webp_support={self.webp_support} />
                <Footer />

                <CursorPointer />
                <ScrollProgressBar />
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    let window = web_sys::window().expect("Thread should have window");

  /*   let did_add_worker = Closure::wrap(Box::new(|object: &JsValue|{
        if js_sys::Reflect::get(object, &JsValue::from("installing")).is_ok() && !js_sys::Reflect::get(object, &JsValue::from("installing")).unwrap().is_undefined() {
            log::info!("Service worker installing");
        } else if js_sys::Reflect::get(object, &JsValue::from("waiting")).is_ok()  && !js_sys::Reflect::get(object, &JsValue::from("waiting")).unwrap().is_undefined(){
            log::info!("Service worker installed"); 
        } else if js_sys::Reflect::get(object, &JsValue::from("active")).is_ok() && !js_sys::Reflect::get(object, &JsValue::from("active")).unwrap().is_undefined() {
            log::info!("Service worker active");
        }
    }) as Box<dyn FnMut(&JsValue)>);

    let failed_to_add_worker = Closure::wrap(Box::new(move ||{
        log::error!("Failed to add worker");
    }) as Box<dyn FnMut()>); */

    wasm_bindgen_futures::spawn_local(async move {
        let registration = wasm_bindgen_futures::JsFuture::from(window.navigator().service_worker().register("sw.js")).await;

        if let Ok(ref object) = registration {
            if js_sys::Reflect::get(object, &JsValue::from("installing")).is_ok() && !js_sys::Reflect::get(object, &JsValue::from("installing")).unwrap().is_undefined() {
                log::info!("Service worker installing");
            } else if js_sys::Reflect::get(object, &JsValue::from("waiting")).is_ok()  && !js_sys::Reflect::get(object, &JsValue::from("waiting")).unwrap().is_undefined(){
                log::info!("Service worker installed"); 
            } else if js_sys::Reflect::get(object, &JsValue::from("active")).is_ok() && !js_sys::Reflect::get(object, &JsValue::from("active")).unwrap().is_undefined() {
                log::info!("Service worker active");
            }
        } else {
            log::info!("Failed to register service worker")
        }
    });

    utils::dark_mode::install_dark_mode_media_query();
    yew::start_app::<Model>();
}
