use gloo_events::EventListener;
use web_sys::Element;
use web_sys::{Document, Window};
use yew::prelude::*;

pub enum Msg {
    Scroll,
    InitText,
    Click,
}

pub struct ScrollProgressBar {
    progress: u32,
    #[allow(dead_code)]
    scroll_listener: EventListener,
    window: Window,
    document: Document,
    animate: bool,
    text_ref: NodeRef,
    bottom_offset: u32,
}

impl Component for ScrollProgressBar {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let window =
            web_sys::window().expect("scroll bar listener needs to have a window available");
        let document = window.document().expect("should have a document on window");

        let onscroll = ctx.link().callback(|_: Event| Msg::Scroll);

        let scroll_listener =
            EventListener::new(&window, "scroll", move |e| onscroll.emit(e.clone()));

        Self {
            scroll_listener,
            window,
            document,
            progress: 0,
            animate: false,
            text_ref: NodeRef::default(),
            bottom_offset: 105,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Scroll => {
                let scroll_top = self.window.scroll_y().unwrap() as f64;
                let document_height =
                    self.document.document_element().unwrap().client_height() as f64;
                let total_scroll_height =
                    self.document.document_element().unwrap().scroll_height() as f64;

                self.progress =
                    ((scroll_top / (total_scroll_height - document_height)) * 100f64) as u32;
                self.animate = scroll_top >= 100f64;
                true
            }
            Msg::InitText => {
                self.bottom_offset = if let Some(element) = self.text_ref.cast::<Element>() {
                    element.client_width() as u32 + 105
                } else {
                    105
                };
                true
            }
            Msg::Click => {
                self.window.scroll_to_with_scroll_to_options(
                    web_sys::ScrollToOptions::new()
                        .top(0.0)
                        .behavior(web_sys::ScrollBehavior::Smooth),
                );

                false
            }
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            ctx.link()
                .callback(|_: Option<u8>| Msg::InitText)
                .emit(None);
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let progress = format!("height: {}%", self.progress);

        let bottom = format!("bottom: {}px", self.bottom_offset);

        let click = ctx.link().callback(|_| Msg::Click);

        html! {
            <div class={classes!(
                "progressbar",
                if self.animate { Some("animate")} else {None},
            )}>
                <a onclick={click} href="#"><span class="text" ref={self.text_ref.clone()} style={bottom}>{"To Top"}</span></a>
                <span class="line" style={progress}></span>
            </div>
        }
    }
}
