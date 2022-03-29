use crate::components::Progress;
use gloo_events::EventListener;
use web_sys::{Document, Window};
use yew::prelude::*;

pub struct Skills {
    show: bool,
    scroll_listener: Option<EventListener>,
    element_ref: NodeRef,
    window: Window,
    document: Document,
}

pub enum Msg {
    Scroll,
}

impl Component for Skills {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let window =
            web_sys::window().expect("scroll bar listener needs to have a window available");
        let document = window.document().unwrap();
        let onscroll = ctx.link().callback(|_: Event| Msg::Scroll);

        let scroll_listener =
            EventListener::new(&window, "scroll", move |e| onscroll.emit(e.clone()));

        Self {
            show: false,
            scroll_listener: Some(scroll_listener),
            element_ref: NodeRef::default(),
            window,
            document,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Scroll => {
                if !self.show {
                    let scroll_top = self.window.scroll_y().unwrap() as f64;

                    let document_height =
                        self.document.document_element().unwrap().client_height() as f64;
                    let y = self
                        .element_ref
                        .cast::<web_sys::Element>()
                        .unwrap()
                        .get_bounding_client_rect()
                        .y();

                    if y <= (scroll_top + (document_height / 12f64 * 10f64)) {
                        self.scroll_listener = None;
                        self.show = true;
                    }
                }
                self.show
            }
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            ctx.link().callback(|_: Option<u8>| Msg::Scroll).emit(None);
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="tm_section" id="skills" ref={self.element_ref.clone()}>
                <div class="tm_skills">
                    <div class="container">
                        <div class="wrapper">
                            <div class="left">
                                <br />
                                <br />
                                <br />
                                <div class="tm_main_title wow fadeInUp" data-wow-duration="1s" data-align="left">
                                    <span class="orangeText">{"You never known enough"}</span>
                                    <h3>{"Choose your weapons wisely, but be realistic"}</h3>
                                    <p>{"Most common methods for designing websites that work well on desktop is responsive and adaptive
                                        design - "} {format!("{}", self.show)}</p>
                                </div>
                                <div class="dodo_progress wow fadeInUp" data-wow-duration="1s">
                                    <Progress value={85} color={"#f75023"} title={"Illustrator"} show={self.show} />
                                    <Progress value={95} color={"#1cbe59"} title={"Photoshop"} show={self.show} />
                                    <Progress value={75} color={"#8067f0"} title={"Figma"} show={self.show} />
                                </div>
                            </div>
                            <div class="right">
                                <br />
                                <br />
                                <br />
                                <div class="tm_main_title wow fadeInUp" data-wow-duration="1s" data-align="left">
                                    <span class="orangeText"><br /></span>
                                    <h3><br /></h3>
                                    <p><br /><br /><br /><br />
                                    </p>
                                </div>
                                <div class="dodo_progress wow fadeInUp" data-wow-duration="1s">
                                    <Progress value={85} color={"#f75023"} title={"Illustrator"} show={self.show} />
                                    <Progress value={95} color={"#1cbe59"} title={"Photoshop"} show={self.show} />
                                    <Progress value={75} color={"#8067f0"} title={"Figma"} show={self.show} />
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
