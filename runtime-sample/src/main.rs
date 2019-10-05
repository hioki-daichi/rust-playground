use actix_web::*;

fn main() {
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind("127.0.0.1:8080")
        .unwrap()
        .run()
        .unwrap()
}

fn index() -> impl Responder {
    "Welcome!".with_status(http::StatusCode::OK)
}
