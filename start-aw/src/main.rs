use actix_web::{server, App, HttpRequest, Responder};

fn hello_name(req: &HttpRequest) -> impl Responder {
    let to = req.match_info().get("name").unwrap_or("World!");
    format!("Hello, {}", to)
}

fn main() {
    server::new(|| App::new().resource("/", |r| r.f(hello_name)))
        .bind("127.0.0.1:59090")
        .unwrap()
        .run();
}
