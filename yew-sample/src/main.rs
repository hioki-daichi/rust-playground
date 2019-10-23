#![recursion_limit = "256"]

#[macro_use]
extern crate stdweb;

use failure::Error;
use stdweb::unstable::TryFrom;
use stdweb::web::Node;
use yew::format::{Nothing, Text};
use yew::prelude::*;
use yew::services::{
    fetch::{FetchTask, Request, Response},
    ConsoleService, FetchService,
};
use yew::virtual_dom::VNode;

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}

struct Model {
    fetch_service: FetchService,
    console: ConsoleService,
    link: ComponentLink<Model>,
    ft: Option<FetchTask>,
    value: String,
}

enum Msg {
    SendRequest,
    FetchReady(Result<String, Error>),
    Ignore,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            fetch_service: FetchService::new(),
            link,
            console: ConsoleService::new(),
            ft: None,
            value: String::from(""),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SendRequest => {
                let callback = self.link.send_back(move |response: Response<Text>| {
                    let (parts, body) = response.into_parts();
                    if parts.status.is_success() {
                        Msg::FetchReady(body)
                    } else {
                        Msg::Ignore
                    }
                });
                let request = Request::get("http://127.0.0.1:8080/hioki-daichi")
                    .body(Nothing)
                    .unwrap();
                self.ft = Some(self.fetch_service.fetch(request, callback));
            }
            Msg::FetchReady(body) => {
                self.console.log("cCCCCCCCCCCCCCCCCCC");
                let s = body.unwrap();
                self.value = s;
            }
            Msg::Ignore => {
                self.console.log("FFFFFFFFFFFFFF");
                self.console.log("ignore");
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <nav class="menu">
                    <button onclick=|_| Msg::SendRequest>{ "Send" }</button>
                </nav>
                <p>{
                    let v = self.value.as_str();
                    if v == "" {
                        VNode::default()
                    } else {
                        let js_svg = js! {
                            var div = document.createElement("div");
                            div.innerHTML = @{v};
                            console.log(div);
                            return div;
                        };
                        let node = Node::try_from(js_svg).expect("foo");
                        let vnode: Html<Self> = VNode::VRef(node);
                        vnode
                    }
                }</p>
            </div>
        }
    }
}
