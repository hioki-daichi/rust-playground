#[derive(Debug)]
struct Parent(u32, Child, Child);

#[derive(Debug)]
struct Child(u32);

fn f(p: &Parent) {
    println!("{:?}", p);
}

fn main() {
    let p1 = Parent(1, Child(11), Child(12));

    f(&p1); // Parent(1, Child(11), Child(12))

    println!("{:?}", p1); // Parent(1, Child(11), Child(12))
}
