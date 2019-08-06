#[derive(Debug)]
struct Parent(usize, Child, Child);

#[derive(Debug)]
struct Child(usize);

fn f1(p: &Parent) {
    println!("p {:?}", p);
}

fn f2(p: &mut Parent) {
    println!("p {:?}", p);
}

fn main() {
    let mut p = Parent(1, Child(11), Child(12));
    f1(&p);
    f2(&mut p);
    println!("p {:?}", p)
}
