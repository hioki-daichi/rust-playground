use actix_web::*;
use futures::future::ok;
use futures::Future;

fn main() {
    HttpServer::new(|| App::new().route("/", web::get().to_async(index)))
        .bind("127.0.0.1:8080")
        .unwrap()
        .run()
        .unwrap()
}

fn index() -> Box<dyn Future<Item = HttpResponse, Error = actix_web::Error>> {
    Box::new(ok::<_, Error>(
        HttpResponse::Ok().content_type("text/html").body("Hello!"),
    ))
}
