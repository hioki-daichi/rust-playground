use yew::html;
use yew::prelude::*;
use yew::services::reader::{FileData, ReaderService, ReaderTask};
use yew::services::ConsoleService;
use yew::ChangeData;

fn main() {
    yew::start_app::<Model>();
}

struct Model {
    link: ComponentLink<Self>,
    reader: ReaderService,
    reader_tasks: Vec<ReaderTask>,
    console: ConsoleService,
    image_src: Option<String>,
}

enum Msg {
    ChooseFile(ChangeData),
    LoadedFile(FileData),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            reader: ReaderService::new(),
            reader_tasks: vec![],
            console: ConsoleService::new(),
            image_src: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChooseFile(ChangeData::Files(files)) => {
                for file in files {
                    self.reader_tasks.push(
                        self.reader
                            .read_file(file, self.link.send_back(move |v| Msg::LoadedFile(v))),
                    );
                }
            }

            Msg::ChooseFile(ChangeData::Value(_)) => {
                self.console.log("ChangeData::Value");
            }

            Msg::ChooseFile(ChangeData::Select(_)) => {
                self.console.log("ChangeData::Select");
            }

            Msg::LoadedFile(file_data) => {
                let content = base64::encode(&file_data.content);
                self.image_src = Some(format!("data:image/gif;base64,{}", content));
            }
        }

        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <input type="file" onchange = |change_data| Msg::ChooseFile(change_data) />
                <img src={self.image_src.as_ref().unwrap_or(&"".to_string())} />
            </div>
        }
    }
}
