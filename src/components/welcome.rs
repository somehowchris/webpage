use yew::prelude::*;

use crate::constants::{FULL_NAME, GITHUB_URL, LINKEDIN_URL};
use crate::utils::images::build_webp_url;

pub struct Welcome {}

impl Component for Welcome {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="tm_section" id="home">
                <div class="tm_hero">
                    <div class="background" data-img-url={build_webp_url("img/slider/1", "jpg")}></div>
                    <div class="container">
                        <div class="content">
                            <div class="details">
                                <div class="hello">
                                    <h3 class="orangeText">{"Hello, I'm"}</h3>
                                </div>
                                <div class="name">
                                    <h3>{FULL_NAME}</h3>
                                </div>
                                <div class="job">
                                    <p>{"A "}<span class="greenText">{"Software Engineer"}</span>{" From "}<span class="purpleText">{"Switzerland"}</span>
                                    </p>
                                </div>
                                <div class="text">
                                    <p></p>
                                </div>
                                <div class="button">
                                    <div class="tm_button">
                                        <a class="anchor" href="#about"><span>{"About Me"}</span></a>
                                    </div>
                                    <div class="social">
                                        <ul>
                                            <li><a href={GITHUB_URL} target="_blank"><span style="color:transparent;line-height:0;font-size:0;">{"Github"}</span><i class="icon-github-2"></i></a></li>
                                            <li><a href={LINKEDIN_URL} target="_blank"><span style="color:transparent;line-height:0;font-size:0;">{"LinkedIn"}</span><i
                                                        class="icon-linkedin-2"></i></a></li>
                                        </ul>
                                    </div>
                                </div>
                            </div>
                            <div class="avatar">
                                <div class="image">
                                    <img src={build_webp_url("img/me-sqr", "png")} alt="" style="opacity: 0;" />
                                    <span class="skills illustrator anim_moveBottom"><img class="svg" src={build_webp_url("img/svg/skills/tensorflow", "png")}
                                            alt="" /></span>
                                    <span class="skills photoshop anim_moveBottom"><img class="svg" src={build_webp_url("img/svg/skills/vscode", "png")}
                                            alt="" /></span>
                                    <span class="skills figma anim_moveBottom"><img class="svg" src={build_webp_url("img/svg/skills/rust", "png")}
                                            alt="" /></span>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="tm_down">
                        <a class="anchor" href="#about" alt="scroll down">
                            <span style="color:transparent;line-height:0;font-size:0;">{"Scroll down"}</span>
                            <svg width="26px" height="100%" viewBox="0 0 247 390" version="1.1" xmlns="http://www.w3.org/2000/svg"
                                style="fill-rule:evenodd;clip-rule:evenodd;stroke-linecap:round;stroke-linejoin:round;stroke-miterlimit:1.5;">
                                <path id="wheel" d="M123.359,79.775l0,72.843" style="fill:none;stroke:#000;stroke-width:20px;" />
                                <path id="mouse"
                                    d="M236.717,123.359c0,-62.565 -50.794,-113.359 -113.358,-113.359c-62.565,0 -113.359,50.794 -113.359,113.359l0,143.237c0,62.565 50.794,113.359 113.359,113.359c62.564,0 113.358,-50.794 113.358,-113.359l0,-143.237Z"
                                    style="fill:none;stroke:#000;stroke-width:20px;" />
                            </svg>
                        </a>
                    </div>
                </div>
            </div>
        }
    }
}
