#![recursion_limit = "256"]

#[macro_use]
extern crate stdweb;

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
    svg: String,
    username: String,
    is_loading: bool,
}

enum Msg {
    SendRequest,
    FetchReady(String),
    Ignore,
    EditUsername(String),
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
            svg: String::from(""),
            username: String::from(""),
            is_loading: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SendRequest => {
                self.is_loading = true;
                let callback = self.link.send_back(move |response: Response<Text>| {
                    let (parts, body) = response.into_parts();
                    if parts.status.is_success() {
                        Msg::FetchReady(body.unwrap())
                    } else {
                        Msg::Ignore
                    }
                });
                let request = Request::get(format!("http://127.0.0.1:8080/{}", self.username))
                    .body(Nothing)
                    .unwrap();
                self.ft = Some(self.fetch_service.fetch(request, callback));
            }
            Msg::FetchReady(body) => {
                self.is_loading = false;
                self.svg = body;
            }
            Msg::Ignore => {
                self.is_loading = false;
                self.svg = String::from("");
            }
            Msg::EditUsername(username) => {
                self.username = username;
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let js_result = js! {
          var element = document.createElement("div");
          element.innerHTML = @{self.svg.as_str()};
          return element;
        };
        let node = Node::try_from(js_result).expect("failed to convert js_result to node");
        let vnode = VNode::VRef(node);

        let loading_message = if self.is_loading { "loading..." } else { "" };

        html! {
          <div>
            <input type="text" oninput=|e| Msg::EditUsername(e.value) />
            <button class="send" onclick=|_| Msg::SendRequest>{ "Send" }</button>
            <span class="loading">{loading_message}</span>
            { vnode }
          </div>
        }
    }
}
