use web_sys::{Document, Window};
use yew::prelude::*;

use gloo_events::EventListener;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

pub enum Msg {
    Increment,
    Scroll,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub from: u16,
    pub to: u16,
    pub time: u16,
}

pub struct Counter {
    pub counter: u16,
    pub show_counter: bool,
    scroll_listener: Option<EventListener>,
    element_ref: NodeRef,
    window: Window,
    document: Document,
}

impl Counter {
    fn start_counter(&self, ctx: &Context<Self>) {
        let window = web_sys::window().expect("Window needed");

        let on_increment = ctx.link().callback(|_: Option<u8>| Msg::Increment);
        let f = Closure::wrap(Box::new(move || {
            on_increment.emit(None);
        }) as Box<dyn FnMut()>);

        let interval = window
            .set_interval_with_callback_and_timeout_and_arguments_0(
                f.as_ref().unchecked_ref(),
                (ctx.props().time / (ctx.props().to - ctx.props().from)).into(),
            )
            .unwrap();
        f.forget();

        let clear = Closure::wrap(Box::new(move || {
            web_sys::window()
                .expect("Window needed")
                .clear_interval_with_handle(interval);
        }) as Box<dyn FnMut()>);

        window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                clear.as_ref().unchecked_ref(),
                (ctx.props().time + 10).into(),
            )
            .unwrap();
        clear.forget();
    }
}

impl Component for Counter {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let window =
            web_sys::window().expect("scroll bar listener needs to have a window available");
        let document = window.document().unwrap();
        let onscroll = ctx.link().callback(|_: Event| Msg::Scroll);

        let scroll_listener =
            EventListener::new(&window, "scroll", move |e| onscroll.emit(e.clone()));

        Self {
            counter: 0,
            show_counter: false,
            scroll_listener: Some(scroll_listener),
            element_ref: NodeRef::default(),
            window,
            document,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Increment => {
                self.counter += 1;

                true
            }
            Msg::Scroll => {
                if !self.show_counter {
                    let scroll_top = self.window.scroll_y().unwrap() as f64;

                    let document_height =
                        self.document.document_element().unwrap().client_height() as f64;
                    let y = self
                        .element_ref
                        .cast::<web_sys::Element>()
                        .unwrap()
                        .get_bounding_client_rect()
                        .y();

                    if y <= (scroll_top + (document_height / 12f64 * 11f64)) {
                        self.scroll_listener = None;
                        self.show_counter = true;
                        self.start_counter(ctx);
                    }
                }
                self.show_counter
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <h3 ref={self.element_ref.clone()}>
                <span class="tm_counter">{self.counter}</span>
            </h3>
        }
    }
}
