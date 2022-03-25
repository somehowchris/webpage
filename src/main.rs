use gloo_events::EventListener;
use wasm_bindgen::JsCast;
use worker::Worker;
use yew::prelude::*;
use yew::{html, Callback};
use yew_agent::{Dispatched, Dispatcher};
use wasm_bindgen::closure::Closure;

mod init;
mod worker;

const FULL_NAME: &str = "Christof Weickhardt";
const NICK_NAME: &str = "Chris";
const EMAIL_ADDRESS: &str = "christof@weickhardt.ch";
const GITHUB_URL: &str = "https://github.com/somehowchris";
const LINKEDIN_URL: &str = "https://linkedin.com/in/christofweickhardt";

enum Msg {
    Scroll,
}

struct Model {
    progress: f64,
    custard_listener: Option<EventListener>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            progress: 0f64,
            custard_listener: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Scroll => {
                let window = web_sys::window().expect("no global `window` exists");
                let document = window.document().expect("should have a document on window");

                let window_height: f64 = window.outer_height().unwrap().as_f64().unwrap();
                let scroll_top = window.scroll_y().unwrap();
                let document_height: i32 = document.document_element().unwrap().client_height();

                self.progress = (f64::from(scroll_top)
                    / (f64::from(document.document_element().unwrap().scroll_height())
                        - f64::from(document_height)))
                    * 100f64;

                true
            }
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            init::tm_stuff();

            if let Some(element) = web_sys::window() {
                // Create your Callback as you normally would
                let onscroll = ctx.link().callback(|_: Event| Msg::Scroll);

                let listener =
                    EventListener::new(&element, "scroll", move |e| onscroll.emit(e.clone()));
                self.custard_listener = Some(listener);
            }

            if let Some(window) = web_sys::window() {
                let query_result = window.match_media("(prefers-color-scheme: dark)");
                if let Ok(Some(mql)) = query_result {

                    let f = Closure::wrap(Box::new(move || {
                        if let Some(window) = web_sys::window() {
                            let query_result = window.match_media("(prefers-color-scheme: dark)");
                            if let Ok(Some(mql)) = query_result {
                                let body = window.document().unwrap().body().expect("document should have a body");
                                let classes = body.class_list();

                                let array = js_sys::Array::new();

                                array.push(&wasm_bindgen::JsValue::from_str("dark"));

                                if !mql.matches() && classes.contains("dark") {
                                    classes.remove(&array).unwrap();
                                } else if !classes.contains("dark") {
                                    classes.add(&array).unwrap();
                                }
                            }
                        }
                    }) as Box<dyn FnMut()>);
                    mql.set_onchange(Some(f.as_ref().unchecked_ref()));
                    f.forget();
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <div>
                    <div id="preloader">
                        <div class="loader_line"></div>
                    </div>
                </div>
                <div class="tm_all_wrap" data-magic-cursor="show">
                    {
                        html!{
                            <div class="tm_mobile_menu">
                                <div class="mobile_menu_inner">
                                    <div class="mobile_in">
                                        <div class="logo">
                                        </div>
                                        <div class="trigger">
                                            <div class="hamburger hamburger--slider">
                                                <div class="hamburger-box">
                                                    <div class="hamburger-inner"></div>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                                <div class="dropdown">
                                    <div class="dropdown_inner">
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
                        }
                    }

                    {
                        html!{
                            <div class="tm_header">
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

                    {
                        html!{
                            <div class="tm_section" id="home">
                        <div class="tm_hero">
                            <div class="background" data-img-url={init::build_webp_url("img/slider/1", "jpg")}></div>
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
                                            <img src={init::build_webp_url("img/me-sqr", "png")} alt="" style="opacity: 0;" />
                                            <span class="skills illustrator anim_moveBottom"><img class="svg" src={init::build_webp_url("img/svg/skills/tensorflow", "png")}
                                                    alt="" /></span>
                                            <span class="skills photoshop anim_moveBottom"><img class="svg" src={init::build_webp_url("img/svg/skills/vscode", "png")}
                                                    alt="" /></span>
                                            <span class="skills figma anim_moveBottom"><img class="svg" src={init::build_webp_url("img/svg/skills/rust", "png")}
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

                    {
                        html!{
                            <div class="tm_section" id="about">
                                <div class="tm_about">
                                    <div class="container">
                                        <div class="wrapper">
                                            <div class="left">
                                                <div class="image">
                                                    <img src={init::build_webp_url("img/me-sqr", "png")} alt="" style="border-radius: 12px" />
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
                                    <div class="brush_1 wow fadeInLeft" data-wow-duration="1s"><img src={init::build_webp_url("img/brushes/about/1", "png")} alt="" /></div>
                                    <div class="brush_2 wow fadeInRight" data-wow-duration="1s"><img src={init::build_webp_url("img/brushes/about/2", "png")} alt="" /></div>
                                </div>
                            </div>

                        }
                    }

                    {
                        html!{

                            <div class="tm_section">
                                <div class="tm_skills">
                                    <div class="container">
                                        <div class="wrapper">
                                            <div class="left">
                                                <br />
                                                <br />
                                                <br />
                                                <div class="tm_main_title wow fadeInUp" data-wow-duration="1s" data-align="left">
                                                    <span class="orangeText">{"You never known enough"}</span>
                                                    <h3>{"Choose your weapons wisely, but be realistic"}</h3>
                                                    <p>{"Most common methods for designing websites that work well on desktop is responsive and adaptive
                                                        design"}</p>
                                                </div>
                                                <div class="dodo_progress wow fadeInUp" data-wow-duration="1s">
                                                    <div class="progress_inner" data-value="85" data-color="#f75023">
                                                        <span><span class="label">{"Illustrator"}</span><span class="number">{"85%"}</span></span>
                                                        <div class="background">
                                                            <div class="bar">
                                                                <div class="bar_in"></div>
                                                            </div>
                                                        </div>
                                                    </div>
                                                    <div class="progress_inner" data-value="95" data-color="#1cbe59">
                                                        <span><span class="label">{"Photoshop"}</span><span class="number">{"95%"}</span></span>
                                                        <div class="background">
                                                            <div class="bar">
                                                                <div class="bar_in"></div>
                                                            </div>
                                                        </div>
                                                    </div>
                                                    <div class="progress_inner" data-value="75" data-color="#8067f0">
                                                        <span><span class="label">{"Figma"}</span><span class="number">{"75%"}</span></span>
                                                        <div class="background">
                                                            <div class="bar">
                                                                <div class="bar_in"></div>
                                                            </div>
                                                        </div>
                                                    </div>
                                                </div>
                                            </div>
                                            <div class="right">
                                                <br />
                                                <br />
                                                <br />
                                                <div class="tm_main_title wow fadeInUp" data-wow-duration="1s" data-align="left">
                                                    <span class="orangeText"><br /></span>
                                                    <h3><br /></h3>
                                                    <p><br /><br /><br /><br />
                                                    </p>
                                                </div>
                                                <div class="dodo_progress wow fadeInUp" data-wow-duration="1s">
                                                    <div class="progress_inner" data-value="85" data-color="#f75023">
                                                        <span><span class="label">{"Illustrator"}</span><span class="number">{"85%"}</span></span>
                                                        <div class="background">
                                                            <div class="bar">
                                                                <div class="bar_in"></div>
                                                            </div>
                                                        </div>
                                                    </div>
                                                    <div class="progress_inner" data-value="95" data-color="#1cbe59">
                                                        <span><span class="label">{"Photoshop"}</span><span class="number">{"95%"}</span></span>
                                                        <div class="background">
                                                            <div class="bar">
                                                                <div class="bar_in"></div>
                                                            </div>
                                                        </div>
                                                    </div>
                                                    <div class="progress_inner" data-value="75" data-color="#8067f0">
                                                        <span><span class="label">{"Figma"}</span><span class="number">{"75%"}</span></span>
                                                        <div class="background">
                                                            <div class="bar">
                                                                <div class="bar_in"></div>
                                                            </div>
                                                        </div>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        }
                    }

                    {
                        html!{

                            <div class="tm_section" id="service">
                                <div class="tm_services">
                                    <div class="container">
                                        <div class="tm_main_title" data-align="center">
                                            <span class="orangeText">{"Projects"}</span>
                                            <h3>{"What I Do for Clients"}</h3>
                                            <p>{"Most common methods for designing websites that work well on desktop is responsive and adaptive design"}
                                            </p>
                                        </div>
                                        <div class="service_list">
                                            <ul>
                                                <li class="wow fadeInLeft" data-wow-duration="1s">
                                                    <div class="list_inner tilt-effect" style="background-color: #AED2D3CC;">
                                                        <div class="title">
                                                            <h3>{"Creative Design"}</h3>
                                                            <span class="price">{"Starts from "}<span>{"$99"}</span></span>
                                                        </div>
                                                        <div class="text">
                                                            <p>{"Web design refers to the design of websites that are displayed on the internet. It usually refers
                                                                to the user experience aspects of website development"}</p>
                                                        </div>
                                                        <img class="popup_service_image" src={init::build_webp_url("img/service/1", "jpg")} alt="" />
                                                        <div class="service_hidden_details">
                                                            <div class="service_popup_informations">
                                                                <div class="descriptions">
                                                                    <p>{"Dizme is a leading web design agency with an award-winning design team that creates
                                                                    innovative, effective websites that capture your brand, improve your conversion rates, and
                                                                    maximize your revenue to help grow your business and achieve your goals."}</p>
                                                                    <p>{"In today’s digital world, your website is the first interaction consumers have with your
                                                                    business. That's why almost 95 percent of a user’s first impression relates to web design.
                                                                    It’s also why web design services can have an immense impact on your company’s bottom line."}
                                                                    </p>
                                                                    <p>{"That’s why more companies are not only reevaluating their website’s design but also
                                                                    partnering with Kura, the web design agency that’s driven more than $2.4 billion in revenue
                                                                    for its clients. With over 50 web design awards under our belt, we're confident we can design
                                                                    a custom website that drives sales for your unique business."}</p>
                                                                </div>
                                                            </div>
                                                        </div>
                                                    </div>
                                                </li>
                                                <li class="wow fadeInRight" data-wow-duration="1s" data-wow-delay="0.2s">
                                                    <div class="list_inner tilt-effect" style="background-color: #C8EACDCC;">
                                                        <div class="title">
                                                            <h3>{"Graphic Design"}</h3>
                                                            <span class="price">{"Starts from "}<span>{"$199"}</span></span>
                                                        </div>
                                                        <div class="text">
                                                            <p>{"Web design refers to the design of websites that are displayed on the internet. It usually refers
                                                                to the user experience aspects of website development"}</p>
                                                        </div>
                                                        <img class="popup_service_image" src={init::build_webp_url("img/service/2", "jpg")} alt="" />
                                                        <div class="service_hidden_details">
                                                            <div class="service_popup_informations">
                                                                <div class="descriptions">
                                                                    <p>{"Dizme is a leading web design agency with an award-winning design team that creates
                                                                    innovative, effective websites that capture your brand, improve your conversion rates, and
                                                                    maximize your revenue to help grow your business and achieve your goals."}</p>
                                                                    <p>{"In today’s digital world, your website is the first interaction consumers have with your
                                                                    business. That's why almost 95 percent of a user’s first impression relates to web design.
                                                                    It’s also why web design services can have an immense impact on your company’s bottom line."}
                                                                    </p>
                                                                    <p>{"That’s why more companies are not only reevaluating their website’s design but also
                                                                    partnering with Kura, the web design agency that’s driven more than $2.4 billion in revenue
                                                                    for its clients. With over 50 web design awards under our belt, we're confident we can design
                                                                    a custom website that drives sales for your unique business."}</p>
                                                                </div>
                                                            </div>
                                                        </div>
                                                    </div>
                                                </li>
                                                <li class="wow fadeInLeft" data-wow-duration="1s">
                                                    <div class="list_inner tilt-effect" style="background-color: #C9D1EDCC;">
                                                        <div class="title">
                                                            <h3>{"UI/UX Design"}</h3>
                                                            <span class="price">{"Starts from "}<span>{"$299"}</span></span>
                                                        </div>
                                                        <div class="text">
                                                            <p>{"Web design refers to the design of websites that are displayed on the internet. It usually refers
                                                                to the user experience aspects of website development"}</p>
                                                        </div>
                                                        <img class="popup_service_image" src={init::build_webp_url("img/service/3", "jpg")} alt="" />
                                                        <div class="service_hidden_details">
                                                            <div class="service_popup_informations">
                                                                <div class="descriptions">
                                                                    <p>{"Dizme is a leading web design agency with an award-winning design team that creates
                                                                    innovative, effective websites that capture your brand, improve your conversion rates, and
                                                                    maximize your revenue to help grow your business and achieve your goals."}</p>
                                                                    <p>{"In today’s digital world, your website is the first interaction consumers have with your
                                                                    business. That's why almost 95 percent of a user’s first impression relates to web design.
                                                                    It’s also why web design services can have an immense impact on your company’s bottom line."}
                                                                    </p>
                                                                    <p>{"That’s why more companies are not only reevaluating their website’s design but also
                                                                    partnering with Kura, the web design agency that’s driven more than $2.4 billion in revenue
                                                                    for its clients. With over 50 web design awards under our belt, we're confident we can design
                                                                    a custom website that drives sales for your unique business."}</p>
                                                                </div>
                                                            </div>
                                                        </div>
                                                    </div>
                                                </li>
                                                <li class="wow fadeInRight" data-wow-duration="1s" data-wow-delay="0.2s">
                                                    <div class="list_inner tilt-effect" style="background-color: #F3E2BACC;">
                                                        <div class="title">
                                                            <h3>{"Web Design"}</h3>
                                                            <span class="price">{"Starts from "}<span>{"$399"}</span></span>
                                                        </div>
                                                        <div class="text">
                                                            <p>{"Web design refers to the design of websites that are displayed on the internet. It usually refers
                                                                to the user experience aspects of website development"}</p>
                                                        </div>
                                                        <img class="popup_service_image" src={init::build_webp_url("img/service/4", "jpg")} alt="" />
                                                        <div class="service_hidden_details">
                                                            <div class="service_popup_informations">
                                                                <div class="descriptions">
                                                                    <p>{"Dizme is a leading web design agency with an award-winning design team that creates
                                                                    innovative, effective websites that capture your brand, improve your conversion rates, and
                                                                    maximize your revenue to help grow your business and achieve your goals."}</p>
                                                                    <p>{"In today’s digital world, your website is the first interaction consumers have with your
                                                                    business. That's why almost 95 percent of a user’s first impression relates to web design.
                                                                    It’s also why web design services can have an immense impact on your company’s bottom line."}
                                                                    </p>
                                                                    <p>{"That’s why more companies are not only reevaluating their website’s design but also
                                                                    partnering with Kura, the web design agency that’s driven more than $2.4 billion in revenue
                                                                    for its clients. With over 50 web design awards under our belt, we're confident we can design
                                                                    a custom website that drives sales for your unique business."}</p>
                                                                </div>
                                                            </div>
                                                        </div>
                                                    </div>
                                                </li>
                                            </ul>
                                        </div>
                                    </div>
                                    <div class="brush_1 wow fadeInLeft" data-wow-duration="1s"><img src={init::build_webp_url("img/brushes/service/5", "png")} alt="" /></div>
                                    <div class="brush_2 wow zoomIn" data-wow-duration="1s"><img src={init::build_webp_url("img/brushes/service/6", "png")} alt="" /></div>
                                </div>
                            </div>
                        }
                    }

                    {
                        html!{
                            <div class="tm_section" style="margin-bottom:96px;">
                                <div class="tm_subscribe">
                                    <div class="container">
                                        <div class="inner">
                                            <div class="background">
                                                <div class="dots" data-img-url={init::build_webp_url("img/subscribe/dots", "jpg")}></div>
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

                    {
                        html!{
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
                    <div class="mouse-cursor cursor-outer"></div>
                    <div class="mouse-cursor cursor-inner"></div>

                    {
                        html!{
                            <div class="progressbar">
                                <a href="#"><span class="text">{"To Top"}</span></a>
                                <span class="line" style={format!("height: {}%", self.progress)}></span>
                            </div>
                        }
                    }
                </div>
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}
