use std::convert::Into;
fn foo(min: impl Into<Option<usize>>, max: impl Into<Option<usize>>) -> [usize; 2] {
    let a = min.into().unwrap_or(1);
    let b = max.into().unwrap_or(100);
    [a, b]
}

fn main() {
    println!("{:?}", foo(30, 70)); // [30, 70]
    println!("{:?}", foo(30, None)); // [30, 100]
    println!("{:?}", foo(None, 70)); // [1, 70]
    println!("{:?}", foo(None, None)); // [1, 100]
}
