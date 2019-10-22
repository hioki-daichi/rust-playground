use actix_web::*;
use futures::Future;

fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(web::resource("/").route(web::get().to_async(handler))))
        .bind("127.0.0.1:8080")?
        .run()
}

struct Foo<'a> {
    a: &'a str,
}

impl<'a> Foo<'a> {
    fn bar(&self) -> impl Future<Item = String, Error = ()> {
        futures::future::ok(format!("{}{}", self.a, "Z"))
    }
}

fn handler() -> impl Future<Item = HttpResponse, Error = ()> {
    let foo = Foo { a: "A" };
    foo.bar()
        .map(|s| HttpResponse::Ok().content_type("text/plain").body(s))
}
