use futures::future::ok;
use futures::Future;
use std::error::Error;

fn my_fut() -> impl Future<Item = u32, Error = Box<dyn Error + 'static>> {
    ok(100)
}

fn my_fut_squared(i: u32) -> impl Future<Item = u32, Error = Box<dyn Error + 'static>> {
    ok(i * i)
}

fn main() {
    println!("{}", "1");
    let b2 = my_fut().and_then(|b1| {
        println!("{}", "2");
        let x = my_fut_squared(b1);
        println!("{}", "3");
        x
    });
    println!("{}", "4");
    println!("{:?}", b2.wait());
    println!("{}", "5");
} // 1 4 2 3 Ok(10000) 5
