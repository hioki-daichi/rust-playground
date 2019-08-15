#[derive(Debug)]
struct Parent(u32, Child, Child);

#[derive(Debug)]
struct Child(u32);

fn f() -> Parent {
    let p = Parent(1, Child(11), Child(12));
    p
}

fn main() {
    let p1 = Some(Parent(1, Child(11), Child(12)));
    match p1 {
        Some(p2) => println!("{:?}", p2), // borrow of moved value: `p1`    value moved here    note: move occurs because value has type `Parent`, which does not implement the `Copy` trait
        None => println!("none"),
    }
    println!("{:?}", p1); // borrow of moved value: `p1`    value borrowed here after partial move    note: move occurs because value has type `Parent`, which does not implement the `Copy` trait
}
