use gloo_events::EventListener;
use yew::prelude::*;
use web_sys::Window;

pub enum Msg {
    Scroll,
}

pub struct Header {
    #[allow(dead_code)]
    scroll_listener: EventListener,
    animate: bool,
    window: Window,
}

impl Component for Header {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let window =
            web_sys::window().expect("scroll bar listener needs to have a window available");
        let onscroll = ctx.link().callback(|_: Event| Msg::Scroll);

        let scroll_listener =
            EventListener::new(&window, "scroll", move |e| onscroll.emit(e.clone()));

        Self {
            scroll_listener,
            window,
            animate: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Scroll => {
                let scroll_top = self.window.scroll_y().unwrap() as f64;
                
                self.animate = scroll_top >= 100f64;

                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class={classes!(
                "tm_header",
                if self.animate { Some("animate")} else {None},
            )}>
                <div class="container">
                    <div class="inner">
                        <div class="logo">
                            <a href="#"></a>
                        </div>
                        <div class="menu">
                            <ul class="anchor_nav">
                                <li class="current"><a href="#home">{"Home"}</a></li>
                                <li><a href="#about">{"About"}</a></li>
                                <li><a href="#portfolio">{"Portfolio"}</a></li>
                                <li><a href="#service">{"Service"}</a></li>
                                <li><a href="#blog">{"Blog"}</a></li>
                                <li><a href="#contact">{"Contact"}</a></li>
                            </ul>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
