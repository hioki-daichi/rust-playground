use actix_web::*;

fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(web::resource("/").route(web::get().to(handler))))
        .bind("127.0.0.1:8080")?
        .run()
}

struct Foo<'a> {
    a: &'a str,
}

impl<'a> Foo<'a> {
    async fn bar(&self) -> Result<String, ()> {
        Ok(format!("{}{}", self.a, "Z"))
    }
}

async fn handler() -> Result<HttpResponse, ()> {
    let foo = Foo { a: "A" };
    foo.bar()
        .await
        .map(|s| HttpResponse::Ok().content_type("text/plain").body(s))
}
