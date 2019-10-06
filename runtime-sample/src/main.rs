use actix_web::client::Client;
use actix_web::*;
use futures::future::ok;
use futures::Future;
use serde_derive::*;

#[derive(Deserialize)]
struct Info {
    username: String,
}

struct Config {
    github_url: &'static str,
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .data(Config {
                github_url: "https://github.com",
            })
            .service(web::resource("/{username}").route(web::get().to_async(handler)))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .unwrap()
}

fn handler(
    config: web::Data<Config>,
    info: web::Path<Info>,
) -> impl Future<Item = HttpResponse, Error = ()> {
    let url = format!("{}/{}.keys", config.github_url, info.username);

    send_request(url).and_then(|body| ok(HttpResponse::Ok().content_type("text/html").body(body)))
}

fn send_request(url: String) -> impl Future<Item = String, Error = ()> {
    Client::default()
        .get(url)
        .send()
        .map_err(|e| {
            println!("{:?}", e);
            ()
        })
        .and_then(|mut response| {
            ok(String::from_utf8(response.body().wait().unwrap().to_vec()).unwrap())
        })
}
