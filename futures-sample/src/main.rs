use failure::{err_msg, Error, Fail};
use futures::Future;

fn main() {
    println!("{:?}", f(100).wait());
}

#[derive(Debug, Fail)]
#[fail(display = "my wrapping error")]
struct WrappingError(#[fail(cause)] Error);

fn f(n: u32) -> impl Future<Item = u32, Error = Error> {
    add1(n).and_then(|b| mul2(b))
}

fn add1(n: u32) -> impl Future<Item = u32, Error = Error> {
    futures::future::ok(n + 1)
}

fn mul2(n: u32) -> impl Future<Item = u32, Error = Error> {
    futures::future::ok(n * 2)
}

fn cause_err(s: String) -> impl Future<Item = u32, Error = Error> {
    futures::future::err(failure::Error { WrappingError.into() })
}
