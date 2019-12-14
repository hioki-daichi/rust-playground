use yew::prelude::*;
use yew::services::ConsoleService;
use yew::ChangeData;

fn main() {
    yew::start_app::<Model>();
}

struct Model {
    #[allow(dead_code)]
    console: ConsoleService,
}

enum Msg {
    ChooseFile(ChangeData),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Model {
            console: ConsoleService::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChooseFile(change_data) => {
                self.console.log(format!("{:?}", change_data).as_str());
            }
        }

        #[allow(unreachable_code)]
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <input type="file" onchange=|change_data| Msg::ChooseFile(change_data) />
            </div>
        }
    }
}
