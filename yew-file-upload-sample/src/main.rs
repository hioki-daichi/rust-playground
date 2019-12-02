use yew::prelude::*;

fn main() {
    yew::start_app::<Model>();
}

struct Model {}

enum Msg {}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Model {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <strong>{{ "Hello, World!" }}</strong>
        }
    }
}
