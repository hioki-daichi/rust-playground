use actix_rt::System;
use actix_web::client::Client;
use failure::{Error, Fail};
use futures::{future::ok, lazy, Future};

fn main() {
    f();
}

fn f() -> impl Future<Item = String, Error = Error> {
    let response_body = System::new("test")
        .block_on(lazy(|| if true { send_request() } else { send_request() }))
        .unwrap();
    println!("{:?}", response_body);
    ok(response_body)
}

fn send_request() -> impl Future<Item = String, Error = Error> {
    Client::default()
        .get("https://github.com/hioki-daichi.keys")
        .send()
        .map_err(|e| MyError::A(format!("{}", e)).into())
        .and_then(|mut response| {
            ok(String::from_utf8(response.body().wait().unwrap().to_vec()).unwrap())
        })
}

#[derive(Debug, Fail)]
enum MyError {
    #[fail(display = "A: {}", _0)]
    A(String),
}
