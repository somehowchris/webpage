use yew::prelude::*;

use crate::utils::images::build_webp_url;

pub struct About {}

impl Component for About {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="tm_section" id="about">
                <div class="tm_about">
                    <div class="container">
                        <div class="wrapper">
                            <div class="left">
                                <div class="image">
                                    <img src={build_webp_url("img/me-sqr", "png")} alt="" style="border-radius: 12px" />
                                    <div class="numbers year" style="opacity: 0.99;">
                                        <div class="wrapper">
                                            <h3><span class="tm_counter" data-from="0" data-to="12" data-speed="2000">{0}</span></h3>
                                            <span class="name">{"Years of"}<br />{"Experience"}</span>
                                        </div>
                                    </div>
                                    <div class="numbers project" style="opacity: 0.99;">
                                        <div class="wrapper">
                                            <h3><span class="tm_counter" data-from="0" data-to="6" data-speed="2000">{0}</span></h3>
                                            <span class="name">{"Open-Source"} <br />{"Projects"}</span>
                                        </div>
                                    </div>
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
                    <div class="brush_1 wow fadeInLeft" data-wow-duration="1s"><img src={build_webp_url("img/brushes/about/1", "png")} alt="" /></div>
                    <div class="brush_2 wow fadeInRight" data-wow-duration="1s"><img src={build_webp_url("img/brushes/about/2", "png")} alt="" /></div>
                </div>
            </div>
        }
    }
}
