fn main() {
    let double: fn(i32) -> i32 = |x: i32| x + x;
    println!("{}", double(1))
}
