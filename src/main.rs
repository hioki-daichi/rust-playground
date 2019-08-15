// Copy, Clone トレイトを実装している
#[derive(Copy, Clone, Debug)]
struct Parent(u32, Child, Child);

// Copy, Clone トレイトを実装している
#[derive(Copy, Clone, Debug)]
struct Child(u32);

fn main() {
    let p1 = Parent(1, Child(11), Child(12));
    let p2 = p1;
    println!("{:?} {:?}", p1, p2);
}
