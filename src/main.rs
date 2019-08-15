#[derive(Debug)]
struct Parent(u32, Child, Child);

#[derive(Debug)]
struct Child(u32);

fn f(p: &mut Parent) {
    println!("{:?}", p);
}

fn main() {
    let mut p1 = Parent(1, Child(11), Child(12));

    f(&mut p1); // Parent(1, Child(11), Child(12))

    println!("{:?}", p1); // Parent(1, Child(11), Child(12))
}
