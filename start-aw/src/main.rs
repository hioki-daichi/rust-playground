fn hello(req: &actix_web::HttpRequest) -> impl actix_web::Responder {
    let to = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", to)
}

fn main() {
    actix_web::server::new(|| {
        actix_web::App::new()
            .resource("/", |r| r.f(hello))
            .resource("/{name}", |r| r.f(hello))
    })
    .bind("localhost:3000")
    .expect("Can not bind to port 3000")
    .run();
}
