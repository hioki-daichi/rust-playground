use std::fmt;

#[derive(Debug)]
enum Either<A, B> {
    A(A),
    B(B),
}

impl<A, B> fmt::Display for Either<A, B>
where
    A: fmt::Display,
    B: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Either::A(a) => a.fmt(f),
            Either::B(b) => b.fmt(f),
        }
    }
}

fn main() {
    let mut v: Vec<Either<bool, i32>> = vec![];
    v.push(Either::A(true));
    v.push(Either::B(1i32));
    for e in v {
        println!("{:?}", e);
    }
    // A(true)
    // B(1)
}
