use failure::Error;
use serde::*;
use serde_json::json;
use yew::format::Json;
use yew::prelude::*;
use yew::services::{
    fetch::{FetchService, FetchTask, Request, Response},
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
    fetch_service: FetchService,
    fetch_task: Option<FetchTask>,
}

enum Msg {
    ChooseFile(ChangeData),
    FileLoaded(FileData),
    RequestCompleted(Video),
    RequestFailed,
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
            fetch_service: FetchService::new(),
            fetch_task: None,
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
                let encoded_data = base64::encode(&file_data.content);

                let request_body = json!({
                    "query":
                        format!(
                            "mutation {{ registerVideo(key: \"{}\", data: \"{}\") {{ src }} }}",
                            file_data.name, encoded_data
                        )
                });

                let request = Request::post("http://localhost:8080/graphql")
                    .header("Content-Type", "application/json")
                    .body(Json(&request_body))
                    .expect("Failed to build request.");

                let callback =
                    self.link
                        .send_back(move |response: GraphQLResponse<RegisterVideoResponse>| {
                            let (meta, Json(response_body)) = response.into_parts();
                            if meta.status.is_success() {
                                let video = response_body.unwrap().data.registerVideo;
                                Msg::RequestCompleted(video)
                            } else {
                                Msg::RequestFailed
                            }
                        });

                let fetch_task = self.fetch_service.fetch(request, callback);

                self.fetch_task = Some(fetch_task);
            }

            Msg::RequestCompleted(video) => {
                self.console.log(format!("{:?}", video).as_str());
            }

            Msg::RequestFailed => {
                self.console.log("RequestFailed");
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

type GraphQLResponse<T> = Response<Json<Result<ResponseData<T>, Error>>>;

#[derive(Deserialize)]
struct ResponseData<T> {
    data: T,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct RegisterVideoResponse {
    registerVideo: Video,
}

#[derive(Debug, Deserialize)]
struct Video {
    src: String,
}
