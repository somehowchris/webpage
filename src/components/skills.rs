use yew::prelude::*;

pub struct Skills {}

impl Component for Skills {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="tm_section" id="skills">
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
                                        <span><span class="label">{"Illustrator"}</span><span class="number" style="right:100%">{"85%"}</span></span>
                                        <div class="background">
                                            <div class="bar">
                                                <div class="bar_in"></div>
                                            </div>
                                        </div>
                                    </div>
                                    <div class="progress_inner" data-value="95" data-color="#1cbe59">
                                        <span><span class="label">{"Photoshop"}</span><span class="number" style="right:100%">{"95%"}</span></span>
                                        <div class="background">
                                            <div class="bar">
                                                <div class="bar_in"></div>
                                            </div>
                                        </div>
                                    </div>
                                    <div class="progress_inner" data-value="75" data-color="#8067f0">
                                        <span><span class="label">{"Figma"}</span><span class="number" style="right:100%">{"75%"}</span></span>
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
                                        <span><span class="label">{"Illustrator"}</span><span class="number" style="right:100%">{"85%"}</span></span>
                                        <div class="background">
                                            <div class="bar">
                                                <div class="bar_in"></div>
                                            </div>
                                        </div>
                                    </div>
                                    <div class="progress_inner" data-value="95" data-color="#1cbe59">
                                        <span><span class="label">{"Photoshop"}</span><span class="number" style="right:100%">{"95%"}</span></span>
                                        <div class="background">
                                            <div class="bar">
                                                <div class="bar_in"></div>
                                            </div>
                                        </div>
                                    </div>
                                    <div class="progress_inner" data-value="75" data-color="#8067f0">
                                        <span><span class="label">{"Figma"}</span><span class="number" style="right:100%">{"75%"}</span></span>
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
}
