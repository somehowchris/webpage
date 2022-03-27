use yew::prelude::*;

use crate::constants::{EMAIL_ADDRESS, FULL_NAME};

pub struct Footer {}

impl Component for Footer {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="tm_section">
                <div class="tm_copyright">
                    <div class="container">
                        <div class="inner">
                            <div class="left wow fadeInLeft" data-wow-duration="1s">
                                <h3 class="orangeText">{FULL_NAME}</h3>
                                <a href={format!("mailto:{}", EMAIL_ADDRESS)}>{EMAIL_ADDRESS}</a>
                                <p><br /></p>
                                <p>{"© All rights are reserved | 2022"}</p>
                            </div>
                            <div class="right wow fadeInRight" data-wow-duration="1s">
                                <ul>
                                    <h3><br /></h3>
                                    <p><br /></p>
                                    <li><a href="#">{"Contact"}</a></li>
                                </ul>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
