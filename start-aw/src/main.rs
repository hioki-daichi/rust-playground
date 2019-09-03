use actix_web::{server, App, HttpResponse};

fn main() {
    server::new(|| App::new().resource("/", |r| r.f(|_| HttpResponse::Ok())))
        .bind("127.0.0.1:59090")
        .unwrap()
        .run();
}
