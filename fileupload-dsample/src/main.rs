use yew::prelude::*;
use yew::services::{reader::ReaderService, ConsoleService};
use yew::ChangeData;

fn main() {
    yew::start_app::<Model>();
}

struct Model {
    #[allow(dead_code)]
    console: ConsoleService,
    reader_service: ReaderService,
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
            reader_service: ReaderService::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChooseFile(change_data) => {
                if let ChangeData::Files(files) = change_data {
                    for file in files {
                        let _reader_task = self
                            .reader_service
                            .read_file(file, Callback::from(|_| panic!()));
                    }
                }
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
