use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub value: u16,
    pub color: &'static str,
    pub title: &'static str,
    pub show: bool,
}

pub struct Progress {
    progress_label: String,
    show_label_opened: bool,
    changed: bool
}

pub enum Msg {
    ShowOpened,
}

impl Component for Progress {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            progress_label: format!("{}%", ctx.props().value),
            show_label_opened: false,
            changed: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ShowOpened => {
                self.show_label_opened = true;

                true
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        if !self.changed {
            self.changed = true;

            let window = web_sys::window().expect("Window needed");

            let on_opened = ctx.link().callback(|_: Option<u8>| Msg::ShowOpened);

            let clear = Closure::wrap(Box::new(move || on_opened.emit(None)) as Box<dyn FnMut()>);
            window
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    clear.as_ref().unchecked_ref(),
                    500.into(),
                )
                .unwrap();

            clear.forget();
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let progress_style = if ctx.props().show {
            format!(
                "right: {}%; color: {}",
                100 - ctx.props().value,
                ctx.props().color
            )
        } else {
            "".to_string()
        };

        let bar_style = if ctx.props().show {
            Some(format!(
                "width: {}%; background-color: {}",
                ctx.props().value,
                ctx.props().color
            ))
        } else {
            None
        };

        html! {
            <div class="progress_inner">
                <span>
                    <span class={classes!("label", if self.show_label_opened {"opened"} else {""})}>
                        {ctx.props().title}
                    </span>
                    <span class="number" style={progress_style}>
                        {&self.progress_label}
                    </span>
                </span>
                <div class="background">
                    <div class={classes!("bar", if ctx.props().show {"open"} else {""})}>
                        <div class="bar_in" style={bar_style}></div>
                    </div>
                </div>
            </div>
        }
    }
}
