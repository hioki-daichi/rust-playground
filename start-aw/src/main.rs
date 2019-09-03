use actix_web::{server, App, Error, FromRequest, HttpRequest, Path, Responder};
use serde_derive::*;

#[derive(Deserialize)]
struct HelloPath {
    name: String,
}

fn foo(req: &HttpRequest) -> Result<String, Error> {
    let to = Path::<HelloPath>::extract(req)?;
    Ok(format!("Hello {}!", &to.name))
}

fn bar(to: Path<HelloPath>) -> impl Responder {
    format!("Hello {}!", &to.name)
}

fn main() {
    server::new(|| {
        App::new()
            .resource("/foo/{name}", |r| r.f(foo))
            .resource("/bar/{name}", |r| r.with(bar))
    })
    .bind("localhost:3000")
    .expect("Can not bind to port 3000")
    .run();
}
