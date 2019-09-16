use actix_web::{client, AsyncResponder, Error, HttpMessage, HttpResponse};
use futures::{future::ok as fut_ok, Future};

fn main() {
    // actix_web::server::new(|| {
    //     actix_web::App::new().route("/", actix_web::http::Method::GET, plain)
    // })
    // .bind("localhost:3000")
    // .unwrap()
    // .run();
}

// fn plain() -> Box<Future<Item = HttpResponse, Error = Error>> {
//     f().and_then(|string: String| Ok(HttpResponse::Ok().content_type("text/plain").body(string)))
//         .responder()
// }

fn f() -> Box<Future<Item = String, Error = Error>> {
    Box::new(
        client::get("http://checkip.amazonaws.com")
            .finish()
            .unwrap()
            .send()
            .map_err(Error::from)
            .and_then(|resp| {
                resp.body().from_err().and_then(|bytes| {
                    let s = String::from_utf8(bytes.to_vec()).unwrap();
                    futures::future::ok(s)
                })
            }),
    )
}
