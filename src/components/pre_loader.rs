use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use regex::Regex;

pub enum Msg {
    Load,
    Hide,
    SetPreloaded,
}

pub struct PreLoader {
    show: bool,
    preloaded: bool,
}

impl Component for PreLoader {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let onload = ctx.link().callback(|_: Option<u8>| Msg::Load);

        let window =
            web_sys::window().expect("scroll bar listener needs to have a window available");

        let load_callback = Closure::wrap(Box::new(move || {
            onload.emit(None);
        }) as Box<dyn FnMut()>);
        window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                load_callback.as_ref().unchecked_ref(),
                500.into(),
            )
            .unwrap();
        load_callback.forget();

        Self {
            show: true,
            preloaded: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Load => {
                let is_mobile = Regex::new(&r"/Android|webOS|iPhone|iPad|iPod|BlackBerry/i").unwrap();

                let window = web_sys::window()
                    .expect("scroll bar listener needs to have a window available");
                if !is_mobile.is_match(&window.navigator().user_agent().unwrap()) {
                    let onpreloaded = ctx.link().callback(|_: Option<u8>| Msg::SetPreloaded);

                    let preloaded_callback = Closure::wrap(Box::new(move || {
                        onpreloaded.emit(None);
                    }) as Box<dyn FnMut()>);
                    window
                        .set_timeout_with_callback_and_timeout_and_arguments_0(
                            preloaded_callback.as_ref().unchecked_ref(),
                            800.into(),
                        )
                        .unwrap();
                    preloaded_callback.forget();

                    let onremove = ctx.link().callback(|_: Option<u8>| Msg::Hide);

                    let remove_callback = Closure::wrap(Box::new(move || {
                        onremove.emit(None);
                    }) as Box<dyn FnMut()>);
                    window
                        .set_timeout_with_callback_and_timeout_and_arguments_0(
                            remove_callback.as_ref().unchecked_ref(),
                            2000.into(),
                        )
                        .unwrap();
                    remove_callback.forget();
                } else {
                    self.show = false;
                }
                true
            },
            Msg::Hide => {
                self.show = false;
                true
            }
            Msg::SetPreloaded => {
                self.preloaded = true;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        if self.show {
            html! {
                <div>
                    <div id="preloader" class={classes!(if self.preloaded {Some("preloaded")} else {None})}>
                        <div class="loader_line"></div>
                    </div>
                </div>
            }
        } else {
            html! {}
        }
    }
}
