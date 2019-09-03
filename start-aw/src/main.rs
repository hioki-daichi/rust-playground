use actix_web::{server, App, Path, Responder};
use serde_derive::*;

#[derive(Deserialize)]
struct ArgPath {
    arg: String,
}

fn handler(s: Path<ArgPath>) -> impl Responder {
    format!("Hello, {}!", &s.arg)
}

fn main() {
    let addr = "127.0.0.1:59090";
    server::new(|| App::new().resource("/{arg}", |r| r.with(handler)))
        .bind(addr)
        .unwrap()
        .run();
}
