use crate::utils::images::build_webp_url;
use yew::prelude::*;

pub struct Projects {}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub webp_support: Option<bool>,
}

impl Component for Projects {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="tm_section" id="projects">
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
                            {
                                if let Some(value) = ctx.props().webp_support {
                                    html!{
                                        <li class="wow fadeInLeft" data-wow-duration="1s">
                                            <div class="list_inner" style="background-color: #AED2D3CC;">
                                                <div class="title">
                                                    <h3>{"Creative Design"}</h3>
                                                    <span class="price">{"Starts from "}<span>{"$99"}</span></span>
                                                </div>
                                                <div class="text">
                                                    <p>{"Web design refers to the design of websites that are displayed on the internet. It usually refers
                                                        to the user experience aspects of website development"}</p>
                                                </div>
                                                <img class="popup_service_image" src={build_webp_url("img/service/1", "jpg", value)} alt="" />
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
                                    }
                                } else {
                                    html!{}
                                }
                            }
                            {
                                if let Some(value) = ctx.props().webp_support {
                                    html!{
                                        <li class="wow fadeInRight" data-wow-duration="1s" data-wow-delay="0.2s">
                                            <div class="list_inner" style="background-color: #C8EACDCC;">
                                                <div class="title">
                                                    <h3>{"Graphic Design"}</h3>
                                                    <span class="price">{"Starts from "}<span>{"$199"}</span></span>
                                                </div>
                                                <div class="text">
                                                    <p>{"Web design refers to the design of websites that are displayed on the internet. It usually refers
                                                        to the user experience aspects of website development"}</p>
                                                </div>
                                                <img class="popup_service_image" src={build_webp_url("img/service/2", "jpg", value)} alt="" />
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
                                    }
                                } else {
                                    html!{}
                                }
                            }
                            {
                                if let Some(value) = ctx.props().webp_support {
                                    html!{
                                        <li class="wow fadeInLeft" data-wow-duration="1s">
                                            <div class="list_inner" style="background-color: #C9D1EDCC;">
                                                <div class="title">
                                                    <h3>{"UI/UX Design"}</h3>
                                                    <span class="price">{"Starts from "}<span>{"$299"}</span></span>
                                                </div>
                                                <div class="text">
                                                    <p>{"Web design refers to the design of websites that are displayed on the internet. It usually refers
                                                        to the user experience aspects of website development"}</p>
                                                </div>
                                                <img class="popup_service_image" src={build_webp_url("img/service/3", "jpg", value)} alt="" />
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
                                    }
                                } else {
                                    html!{}
                                }
                            }
                            {
                                if let Some(value) = ctx.props().webp_support {
                                    html!{
                                        <li class="wow fadeInRight" data-wow-duration="1s" data-wow-delay="0.2s">
                                            <div class="list_inner" style="background-color: #F3E2BACC;">
                                                <div class="title">
                                                    <h3>{"Web Design"}</h3>
                                                    <span class="price">{"Starts from "}<span>{"$399"}</span></span>
                                                </div>
                                                <div class="text">
                                                    <p>{"Web design refers to the design of websites that are displayed on the internet. It usually refers
                                                        to the user experience aspects of website development"}</p>
                                                </div>
                                                <img class="popup_service_image" src={build_webp_url("img/service/4", "jpg", value)} alt="" />
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
                                    }
                                } else {
                                    html!{}
                                }
                            }
                        </ul>
                    </div>
                </div>
                {
                    if let Some(value) = ctx.props().webp_support {
                        html!{
                            <>
                                <div class="brush_1 wow fadeInLeft" data-wow-duration="1s"><img src={build_webp_url("img/brushes/service/5", "png", value)} alt="" /></div>
                                <div class="brush_2 wow zoomIn" data-wow-duration="1s"><img src={build_webp_url("img/brushes/service/6", "png", value)} alt="" /></div>
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
