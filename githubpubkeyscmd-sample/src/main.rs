use actix_rt::System;
use actix_web::client::Client;
use failure::{Error, Fail};
use futures::{future::ok, future::result, lazy, Future};

fn main() {
    System::new("test")
        .block_on(lazy(|| {
            let foo = if true { send_request() } else { send_request() };
            foo.and_then(|response_body| {
                println!("{:?}", response_body);
                ok(())
            })
            .wait()
        }))
        .unwrap();
}

fn send_request() -> impl Future<Item = String, Error = Error> {
    result(
        Client::default()
            .get("https://github.com/hioki-daichi")
            .send()
            .map_err(|e| MyError::A(format!("{}", e)).into())
            .and_then(|mut response| {
                ok(String::from_utf8(response.body().wait().unwrap().to_vec()).unwrap())
            })
            .wait(),
    )
}

#[derive(Debug, Fail)]
enum MyError {
    #[fail(display = "A: {}", _0)]
    A(String),
}
