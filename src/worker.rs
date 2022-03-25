use gloo_timers::callback::Interval;
use serde::{Deserialize, Serialize};
use yew_agent::{HandlerId, Public, Agent, AgentLink};
use yew_agent::Bridged;

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    GetDataFromServer,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    DataFetched,
}

pub enum Msg {
}

pub struct Worker {
    link: AgentLink<Worker>,
    _interval: Interval,
}

impl Agent for Worker {
    type Reach = Public<Self>;
    type Message = Msg;
    type Input = Request;
    type Output = Response;

    fn create(link: AgentLink<Self>) -> Self {
        let duration = 3;

        let interval = {
            let link = link.clone();
            Interval::new(duration, move || {
                let window = web_sys::window().expect("no global `window` exists");
                let document = window.document().expect("should have a document on window");
                let body = document.body().expect("document should have a body");
                let classes = body.class_list();
        
                let array = js_sys::Array::new();
        
                array.push(&wasm_bindgen::JsValue::from_str("dark"));
        
                if classes.contains("dark") {
                    classes.remove(&array);
                } else {
                    classes.add(&array);
                }
            })
        };
        Self {
            link,
            _interval: interval,
        }
    }

    fn update(&mut self, msg: Self::Message) {
    }

    fn handle_input(&mut self, msg: Self::Input, who: HandlerId) {
    }

    fn name_of_resource() -> &'static str {
        "worker.js"
    }
}