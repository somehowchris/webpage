use yew::prelude::*;

pub mod components;
pub mod constants;
pub mod utils;
mod views;

pub use utils::images::build_webp_url;

use yew_router::prelude::*;
use views::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Main /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    utils::service_worker::register_service_worker();
    utils::dark_mode::install_dark_mode_media_query();

    yew::start_app::<App>();
}
