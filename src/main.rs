use yew::prelude::*;

mod components;
pub mod constants;
mod init;
pub mod utils;

pub use utils::images::build_webp_url;

use components::{Contact, CursorPointer, Footer, Header, ScrollProgressBar, About, Welcome, Skills, Projects};

struct Model {}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
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
                <Welcome />
                <About />
                <Skills />
                <Projects />
                <Contact />
                <Footer />

                <CursorPointer />
                <ScrollProgressBar />
            </div>
        }
    }
}

fn main() {
    #[cfg(debug_assertions)]
    wasm_logger::init(wasm_logger::Config::default());

    utils::dark_mode::install_dark_mode_media_query();
    yew::start_app::<Model>();
}
