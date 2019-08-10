fn main() {
    let t1 = ('a', 42);
    // let t2 = t1 as (u32, u8); // non-primitive cast: `(char, i32)` as `(u32, u8)`    note: an `as` expression can only be used to convert between primitive types. Consider using the `From` trait
    let t3 = (t1.0 as u32, t1.1 as u8); // 要素を 1 つずつキャスト
    println!("{:?}", t3); // (97, 42)
}
