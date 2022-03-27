use yew::prelude::*;

use crate::constants::EMAIL_ADDRESS;
use crate::utils::images::build_webp_url;

pub struct Contact {}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub webp_support: Option<bool>,
}

impl Component for Contact {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="tm_section" style="margin-bottom:96px;" id="contact">
                <div class="tm_subscribe">
                    <div class="container">
                        <div class="inner">
                            <div class="background">
                                {
                                    if let Some(value) = ctx.props().webp_support {
                                        html!{
                                            <div class="dots" style={format!("background-image: url({:?})", build_webp_url("img/subscribe/dots", "jpg", value))}></div>
                                        }
                                    } else {
                                        html!{}
                                    }
                                }
                                <div class="overlay"></div>
                            </div>
                            <div class="content">
                                <div class="left wow fadeInLeft" data-wow-duration="1s">
                                    <span class="subtitle">{"Contact Me"}</span>
                                    <h3 class="title">{"I Want To Hear From You"}</h3>
                                </div>
                                <div class="right wow fadeInRight" data-wow-duration="1s">
                                    <div class="field">
                                        <div class="tm_button">
                                            <a class="anchor" href={format!("mailto:{}", EMAIL_ADDRESS)}
                                                style="position:absolute; right:0;"><span>{"E-Mail Me"}</span></a>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>

                        <p>
                            <br />
                        </p>
                    </div>
                </div>
            </div>
        }
    }
}
