use actix_http::Error;
use actix_rt::System;
use futures::{future::lazy, Future};

fn main() -> Result<(), Error> {
    System::new("foo").block_on(lazy(|| {
        actix_web::client::Client::default()
            .get(String::from("https://checkip.amazonaws.com"))
            .send()
            .from_err()
            .and_then(|mut response| {
                response
                    .body()
                    .from_err()
                    .map(|body| println!("{:?}", body))
            })
    }))
}
