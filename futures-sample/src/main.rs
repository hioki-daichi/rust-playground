use failure::{Error, Fail, format_err};

fn main() {
    let fut = f(101);
    println!("{:?}", futures::executor::block_on(fut));
}

async fn f(a: u32) -> Result<u32, Error> {
    let b = add1(a).await?;
    let c = mul3(b).await?;
    cause_err(c).await.map_err(|e| format_err!("display is '{}'", e))
}

async fn add1(a: u32) -> Result<u32, Error> {
    Ok(a + 1)
}

async fn mul3(a: u32) -> Result<u32, Error> {
    Ok(a * 3)
}

#[derive(Debug, Fail)]
#[fail(display = "foobar")]
struct Foo;

async fn cause_err(a: u32) -> Result<u32, Error> {
    if a % 2 == 0 {
        Err(Foo.into())
    } else {
        Ok(a)
    }
}
