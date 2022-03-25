use gloo_events::EventListener;
use web_sys::{Document, Window};
use yew::prelude::*;
use web_sys::Element;

pub enum Msg {
    Scroll,
    InitText,
}

pub struct ScrollProgressBar {
    progress: u32,
    #[allow(dead_code)]
    scroll_listener: EventListener,
    window: Window,
    document: Document,
    animate: bool,
    text_ref: NodeRef,
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
            Msg::InitText => true,
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        ctx.link().callback(|_: Option<u8>| Msg::InitText).emit(None);
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let progress = format!("height: {}%", self.progress);

        let bottom = format!(
            "bottom: {}px",
            if let Some(element) = self.text_ref.cast::<Element>() {
                element.client_width() + 105
            } else {
                105
            }
        );

        html! {
            <div class={classes!(
                "progressbar",
                if self.animate { Some("animate")} else {None},
            )}>
                <a href="#"><span class="text" ref={self.text_ref.clone()}>{"To Top"}</span></a>
                <span class="line" style={progress}></span>
            </div>
        }
    }
}
