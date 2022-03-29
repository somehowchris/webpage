use yew::prelude::*;

use crate::utils;
use super::sections::*;

use crate::components::{
    CursorPointer, Footer, Header, ScrollProgressBar,
};

pub enum Msg {
    WebPSupport(bool),
}

pub struct Main {
    webp_support: Option<bool>,
}

impl Component for Main {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async {
            match utils::images::has_webp_support().await {
                true => Self::Message::WebPSupport(true),
                false => Self::Message::WebPSupport(false),
            }
        });
        Self { webp_support: None }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::WebPSupport(value) => {
                self.webp_support = Some(value);
                true
            }
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