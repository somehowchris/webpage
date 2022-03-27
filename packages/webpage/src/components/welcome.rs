use yew::prelude::*;

use crate::constants::{FULL_NAME, GITHUB_URL, LINKEDIN_URL};
use crate::utils::images::build_webp_url;

pub struct Welcome {}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub webp_support: Option<bool>,
}

impl Component for Welcome {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="tm_section" id="home">
                <div class="tm_hero">
                    {
                        if let Some(value) = ctx.props().webp_support {
                            html!{
                                <div class="background" style={format!("background-image: url({:?})", build_webp_url("img/slider/1", "jpg", value))}></div>
                            }
                        } else {
                            html!{}
                        }
                    }
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
                                            <li><a href={GITHUB_URL} target="_blank"><span style="color:transparent;line-height:0;font-size:0;">{"Github"}</span><img src="img/github.svg" style="height: 24px;top: -4px;position: relative;" class="icon-github-2" /></a></li>
                                            <li><a href={LINKEDIN_URL} target="_blank"><span style="color:transparent;line-height:0;font-size:0;">{"LinkedIn"}</span><i
                                                        class="icon-linkedin-4" style="font-size: 1.5em"></i></a></li>
                                        </ul>
                                    </div>
                                </div>
                            </div>
                            <div class="avatar">
                                {
                                    if let Some(value) = ctx.props().webp_support {
                                        html!{
                                            <div class="image">
                                                <img src={build_webp_url("img/me-sqr", "png", value)} alt="" style="opacity: 0;" />
                                                <span class="skills illustrator anim_moveBottom"><img class="svg" src={build_webp_url("img/svg/skills/tensorflow", "png", value)}
                                                        alt="" /></span>
                                                <span class="skills photoshop anim_moveBottom"><img class="svg" src={build_webp_url("img/svg/skills/vscode", "png", value)}
                                                        alt="" /></span>
                                                <span class="skills figma anim_moveBottom"><img class="svg" src={build_webp_url("img/svg/skills/rust", "png", value)}
                                                        alt="" /></span>
                                            </div>

                                        }
                                    } else {
                                        html!{}
                                    }
                                }
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
