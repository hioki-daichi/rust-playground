use actix_web::{server, App, Error, FromRequest, HttpRequest, Path};
use serde_derive::*;

#[derive(Deserialize)]
struct HelloPath {
    name: String,
}

fn hello_name(req: &HttpRequest) -> Result<String, Error> {
    let to = Path::<HelloPath>::extract(req)?;
    Ok(format!("Hello, {}!", &to.name))
}

fn main() {
    server::new(|| App::new().resource("/{name}", |r| r.f(hello_name)))
        .bind("127.0.0.1:59090")
        .unwrap()
        .run();
}
