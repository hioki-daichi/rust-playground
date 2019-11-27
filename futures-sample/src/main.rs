use failure::{format_err, Error, Fail};

fn main() {
    let fut = f(100);
    println!("{:?}", futures::executor::block_on(fut));
}

async fn f(a: u32) -> Result<u32, Error> {
    let b = add1(a).await?;
    let c = mul3(b).await?;
    cause_err(c).await
}

async fn add1(a: u32) -> Result<u32, Error> {
    Ok(a + 1)
}

async fn mul3(a: u32) -> Result<u32, Error> {
    Ok(a * 3)
}

#[derive(Debug, Fail)]
#[fail(display = "my wrapping error")]
struct WrappingError(#[fail(cause)] Error);

async fn cause_err(a: u32) -> Result<u32, Error> {
    if a % 2 == 0 {
        Err(format_err!("This is even! {:?}", "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"))
    } else {
        Ok(a)
    }
}
