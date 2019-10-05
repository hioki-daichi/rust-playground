use actix_web::client::Client;
use actix_web::*;
use futures::future::ok;
use futures::Future;
use serde_derive::*;

#[derive(Deserialize)]
struct Info {
    username: String,
}

fn main() {
    HttpServer::new(|| {
        App::new().service(web::resource("/{username}").route(web::get().to_async(handler)))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .unwrap()
}

fn handler(info: web::Path<Info>) -> impl Future<Item = HttpResponse, Error = ()> {
    Client::default()
        .get(format!("https://github.com/{}.keys", info.username))
        .send()
        .map_err(|e| {
            println!("{:?}", e);
            ()
        })
        .and_then(|mut response| {
            ok(HttpResponse::Ok()
                .content_type("text/html")
                .body(String::from_utf8(response.body().wait().unwrap().to_vec()).unwrap()))
        })
}
