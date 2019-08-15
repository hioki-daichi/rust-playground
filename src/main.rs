#[derive(Debug)]
struct Parent(u32, Child, Child);

#[derive(Debug)]
struct Child(u32);

fn f(p: Parent) {
    println!("{:?}", p);
}

fn main() {
    let p1 = Parent(1, Child(11), Child(12));

    f(p1);

    println!("{:?}", p1); // borrow of moved value: `p1`    value borrowed here after move
}
