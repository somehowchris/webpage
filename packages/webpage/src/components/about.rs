use yew::prelude::*;

use super::Counter;
use crate::utils::images::build_webp_url;

pub struct About {}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub webp_support: Option<bool>,
}

impl Component for About {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="tm_section" id="about">
                <div class="tm_about">
                    <div class="container">
                        <div class="wrapper">
                            <div class="left">
                                <div class="image">
                                    {
                                        if let Some(value) = ctx.props().webp_support {
                                            html!{
                                                <>
                                                    <img src={build_webp_url("img/me-sqr", "png", value)} alt="" style="border-radius: 12px" />
                                                    <div class="numbers year" style="opacity: 0.99;">
                                                        <div class="wrapper">
                                                            <Counter from={0} to={12} time={2000}/>
                                                            <span class="name">{"Years of"}<br />{"Experience"}</span>
                                                        </div>
                                                    </div>
                                                    <div class="numbers project" style="opacity: 0.99;">
                                                        <div class="wrapper">
                                                            <Counter from={0} to={6} time={2000}/>
                                                            <span class="name">{"Open-Source"} <br />{"Projects"}</span>
                                                        </div>
                                                    </div>
                                                </>
                                            }
                                        } else {
                                            html!{}
                                        }
                                    }
                                </div>
                            </div>
                            <div class="right">
                                <div class="title wow fadeInUp" data-wow-duration="1s">
                                    <span class="orangeText">{"I'm a Developer"}</span>
                                    <h3>{"I love to develop "} <h3 class="greenText"> {"Software"}</h3>
                                    </h3>
                                </div>
                                <div class="text wow fadeInUp" data-wow-duration="1s">
                                    <p>{"I'm a software developer, and I'm very passionate and dedicated to my work. I started developing
                                    software at the age of 10. Not only for learning new things and challenging myself but to build
                                    software to have fun. With my degree of practical software engineering I gained massive knowledge in
                                    DevOps, Artifical Intelligence and a lot more."}
                                    </p>
                                </div>
                                <div class="tm_button wow fadeInUp" data-wow-duration="1s">
                                    <a class="anchor" href="#contact"><span>{"Hire Me"}</span></a>
                                </div>
                            </div>
                        </div>
                    </div>
                    {
                        if let Some(value) = ctx.props().webp_support {
                            html!{
                                <>
                                    <div class="brush_1 wow fadeInLeft" data-wow-duration="1s"><img src={build_webp_url("img/brushes/about/1", "png", value)} alt="" /></div>
                                    <div class="brush_2 wow fadeInRight" data-wow-duration="1s"><img src={build_webp_url("img/brushes/about/2", "png", value)} alt="" /></div>
                                </>
                            }
                        } else {
                            html!{}
                        }
                    }
                </div>
            </div>
        }
    }
}
