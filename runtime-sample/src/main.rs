use actix_web::client::Client;
use actix_web::*;
use failure::{Error, Fail};
use futures::future::ok;
use futures::{future::err, future::Either, Future};
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

    if true {
        Either::A(if true {
            Either::A(
                send_request(url)
                    .map_err(|_e| MyError::A().into())
                    .and_then(|body| ok(HttpResponse::Ok().body(body))),
            )
        } else {
            Either::B(
                send_request(url)
                    .map_err(|_e| MyError::B().into())
                    .and_then(|_body| ok(HttpResponse::Ok().body("unreachable"))),
            )
        })
    } else {
        Either::B(err(MyError::A().into()))
    }
}

fn send_request(url: String) -> impl Future<Item = String, Error = Error> {
    Client::default()
        .get(url)
        .send()
        .map_err(|_e| MyError::C().into())
        .and_then(|mut response| {
            ok(String::from_utf8(response.body().wait().unwrap().to_vec()).unwrap())
        })
}

#[derive(Debug, Fail)]
enum MyError {
    #[fail(display = "A")]
    A(),

    #[fail(display = "B")]
    B(),

    #[fail(display = "C")]
    C(),
}
