use actix_web::client::Client;
use actix_web::*;
use failure::{Error, Fail};
use futures::future::ok;
use futures::{future::Either, Future};
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
) -> impl Future<Item = HttpResponse, Error = Error> {
    let url = format!("{}/{}.keys", config.github_url, info.username);

    let foo = if true {
        Either::A(
            send_request(url)
                .map_err(|e| MyError::A(format!("A: {}", e)).into())
                .and_then(|body| ok(HttpResponse::Ok().content_type("text/html").body(body))),
        )
    } else {
        Either::B(
            send_request(url)
                .map_err(|e| MyError::B(format!("B: {}", e)).into())
                .and_then(|body| ok(HttpResponse::Ok().content_type("text/plain").body(body))),
        )
    };

    foo
}

fn send_request(url: String) -> impl Future<Item = String, Error = Error> {
    Client::default()
        .get(url)
        .send()
        .map_err(|e| MyError::C(format!("{}", e)).into())
        .and_then(|mut response| {
            ok(String::from_utf8(response.body().wait().unwrap().to_vec()).unwrap())
        })
}

#[derive(Debug, Fail)]
enum MyError {
    #[fail(display = "A: {}", _0)]
    A(String),

    #[fail(display = "B: {}", _0)]
    B(String),

    #[fail(display = "C: {}", _0)]
    C(String),
}
