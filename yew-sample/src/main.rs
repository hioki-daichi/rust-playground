#![recursion_limit = "256"]

use yew::prelude::*;
use yew::services::ConsoleService;

fn main() {
    yew::start_app::<Model>();
}

#[derive(Debug)]
struct A {
    a: u32,
}

#[derive(Debug)]
struct Model {
    xs: Vec<A>,
    console: ConsoleService,
}

enum Msg {
    X,
    Y,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Model {
            xs: vec![],
            console: ConsoleService::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::X => {
                self.xs.insert(0, A { a: 1 });
                self.console.log(format!("{:?}", self.xs).as_str());
            }
            Msg::Y => {
                self.xs.insert(0, A { a: 2 });
                self.console.log(format!("{:?}", self.xs).as_str());
            }
        }
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
          <div>
              <button onclick=|_e| Msg::X>{{"XXXXXXXXXX"}}</button>
              <button onclick=|_e| Msg::Y>{{"YYYYYYYYYYYY"}}</button>
          </div>
        }
    }
}
