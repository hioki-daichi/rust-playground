#[derive(Debug)]
struct Parent(u32, Child, Child);

#[derive(Debug)]
struct Child(u32);

fn main() {
    let p = Parent(1, Child(11), Child(12)); // borrow of moved value: `p`    move occurs because `p` has type `Parent`, which does not implement the `Copy` trait

    // move が付いている
    let f = move |x: u32| x + p.0; // borrow of moved value: `p`    variable moved due to use in closure

    // コンパイルエラー
    println!("{} {:?}", f(1), p); // borrow of moved value: `p`    value borrowed here after move
}
