use yew::prelude::*;

enum Msg {
}

struct Model {
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
       true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <div>    
                    <div id="preloader">
                        <div class="loader_line"></div>
                    </div>
                </div>
                <div class="dizme_tm_all_wrap" data-magic-cursor="show">
                    {
                        html!{
                            <div class="dizme_tm_mobile_menu">
                                <div class="mobile_menu_inner">
                                    <div class="mobile_in">
                                        <div class="logo">
                                            <a href="#"></a>
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
                            <div class="dizme_tm_header">
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
                            <div class="dizme_tm_section" id="home">
                        <div class="dizme_tm_hero">
                            <div class="background" data-img-url="img/slider/1.jpg"></div>
                            <div class="container">
                                <div class="content">
                                    <div class="details">
                                        <div class="hello">
                                            <h3 class="orangeText">{"Hello, I'm"}</h3>
                                        </div>
                                        <div class="name">
                                            <h3>{"Christof Weickhardt"}</h3>
                                        </div>
                                        <div class="job">
                                            <p>{"A "}<span class="greenText">{"Software Engineer"}</span>{" From "}<span class="purpleText">{"Switzerland"}</span>
                                            </p>
                                        </div>
                                        <div class="text">
                                            <p></p>
                                        </div>
                                        <div class="button">
                                            <div class="dizme_tm_button">
                                                <a class="anchor" href="#about"><span>{"About Me"}</span></a>
                                            </div>
                                            <div class="social">
                                                <ul>
                                                    <li><a href="https://github.com/somehowchris" target="_blank"><i class="icon-github-1"></i></a></li>
                                                    <li><a href="https://ch.linkedin.com/in/christofweickhardt" target="_blank"><i
                                                                class="icon-linkedin-1"></i></a></li>
                                                </ul>
                                            </div>
                                        </div>
                                    </div>
                                    <div class="avatar">
                                        <div class="image">
                                            <img src="img/me-sqr.png" alt="" style="opacity: 0;" />
                                            <span class="skills illustrator anim_moveBottom"><img class="svg" src="img/svg/skills/tensorflow.png"
                                                    alt="" /></span>
                                            <span class="skills photoshop anim_moveBottom"><img class="svg" src="img/svg/skills/vscode.png"
                                                    alt="" /></span>
                                            <span class="skills figma anim_moveBottom"><img class="svg" src="img/svg/skills/rust.png"
                                                    alt="" /></span>
                                        </div>
                                    </div>
                                </div>
                            </div>
                            <div class="dizme_tm_down">
                                <a class="anchor" href="#about">
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
                            <div class="dizme_tm_section" id="about">
                                <div class="dizme_tm_about">
                                    <div class="container">
                                        <div class="wrapper">
                                            <div class="left">
                                                <div class="image">
                                                    <img src="img/me-sqr.png" alt="" style="border-radius: 12px" />
                                                    <div class="numbers year" style="opacity: 0.99;">
                                                        <div class="wrapper">
                                                            <h3><span class="dizme_tm_counter" data-from="0" data-to="12" data-speed="2000">{0}</span></h3>
                                                            <span class="name">{"Years of"}<br />{"Experience"}</span>
                                                        </div>
                                                    </div>
                                                    <div class="numbers project" style="opacity: 0.99;">
                                                        <div class="wrapper">
                                                            <h3><span class="dizme_tm_counter" data-from="0" data-to="6" data-speed="2000">{0}</span></h3>
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
                                                <div class="dizme_tm_button wow fadeInUp" data-wow-duration="1s">
                                                    <a class="anchor" href="#contact"><span>{"Hire Me"}</span></a>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                    <div class="brush_1 wow fadeInLeft" data-wow-duration="1s"><img src="img/brushes/about/1.png" alt="" /></div>
                                    <div class="brush_2 wow fadeInRight" data-wow-duration="1s"><img src="img/brushes/about/2.png" alt="" /></div>
                                </div>
                            </div>

                        }
                    }

                    {
                        html!{

                            <div class="dizme_tm_section">
                                <div class="dizme_tm_skills">
                                    <div class="container">
                                        <div class="wrapper">
                                            <div class="left">
                                                <br />
                                                <br />
                                                <br />
                                                <div class="dizme_tm_main_title wow fadeInUp" data-wow-duration="1s" data-align="left">
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
                                                <div class="dizme_tm_main_title wow fadeInUp" data-wow-duration="1s" data-align="left">
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

                            <div class="dizme_tm_section" id="service">
                                <div class="dizme_tm_services">
                                    <div class="container">
                                        <div class="dizme_tm_main_title" data-align="center">
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
                                                        <a class="dizme_tm_full_link" href="#"></a>
                                                        <img class="popup_service_image" src="img/service/1.jpg" alt="" />
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
                                                        <a class="dizme_tm_full_link" href="#"></a>
                                                        <img class="popup_service_image" src="img/service/2.jpg" alt="" />
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
                                                        <a class="dizme_tm_full_link" href="#"></a>
                                                        <img class="popup_service_image" src="img/service/3.jpg" alt="" />
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
                                                        <a class="dizme_tm_full_link" href="#"></a>
                                                        <img class="popup_service_image" src="img/service/4.jpg" alt="" />
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
                                    <div class="brush_1 wow fadeInLeft" data-wow-duration="1s"><img src="img/brushes/service/5.png" alt="" /></div>
                                    <div class="brush_2 wow zoomIn" data-wow-duration="1s"><img src="img/brushes/service/6.png" alt="" /></div>
                                </div>
                            </div>
                        }
                    }

                    {
                        html!{
                            <div class="dizme_tm_section" style="margin-bottom:96px;">
                                <div class="dizme_tm_subscribe">
                                    <div class="container">
                                        <div class="inner">
                                            <div class="background">
                                                <div class="dots" data-img-url="img/subscribe/dots.jpg"></div>
                                                <div class="overlay"></div>
                                            </div>
                                            <div class="content">
                                                <div class="left wow fadeInLeft" data-wow-duration="1s">
                                                    <span class="subtitle">{"Contact Me"}</span>
                                                    <h3 class="title">{"I Want To Hear From You"}</h3>
                                                </div>
                                                <div class="right wow fadeInRight" data-wow-duration="1s">
                                                    <div class="field">
                                                        <div class="dizme_tm_button">
                                                            <a class="anchor" href="mailto:christof@weickhardt.ch"
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
                            <div class="dizme_tm_section">
                                <div class="dizme_tm_copyright">
                                    <div class="container">
                                        <div class="inner">
                                            <div class="left wow fadeInLeft" data-wow-duration="1s">
                                                <h3 class="orangeText">{"Christof Weickhardt"}</h3>
                                                <a href="mailto:christof@weickhardt.ch">{"christof@weickhardt.ch"}</a>
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
                                <span class="line"></span>
                            </div>
                        }
                    }
                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
