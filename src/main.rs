fn to_n(n: i32) -> impl Iterator {
    0..n
}

fn main() {
    let a = to_n(3);
    println!("{:?}", a);
}
