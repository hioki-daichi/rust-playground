use yew::prelude::*;
use yew::services::{
    reader::{FileData, ReaderService, ReaderTask},
    ConsoleService,
};
use yew::ChangeData;

fn main() {
    yew::start_app::<Model>();
}

struct Model {
    #[allow(dead_code)]
    console: ConsoleService,
    reader_service: ReaderService,
    reader_tasks: Vec<ReaderTask>,
    link: ComponentLink<Self>,
}

enum Msg {
    ChooseFile(ChangeData),
    FileLoaded(FileData),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            console: ConsoleService::new(),
            reader_service: ReaderService::new(),
            reader_tasks: vec![],
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChooseFile(change_data) => {
                if let ChangeData::Files(files) = change_data {
                    for file in files {
                        let reader_task = self.reader_service.read_file(
                            file,
                            self.link.send_back(|file_data| Msg::FileLoaded(file_data)),
                        );
                        self.reader_tasks.push(reader_task);
                    }
                }
            }

            Msg::FileLoaded(file_data) => {
                self.console.log(format!("{:?}", file_data).as_str());
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
