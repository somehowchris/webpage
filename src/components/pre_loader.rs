use yew::prelude::*;

pub struct PreLoader {}

impl Component for PreLoader {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <div id="preloader">
                    <div class="loader_line"></div>
                </div>
            </div>
        }
    }
}
