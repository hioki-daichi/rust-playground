use std::ops::Drop;

#[derive(Debug)]
struct Parent(u32, Child, Child);

#[derive(Debug)]
struct Child(u32);

impl Drop for Parent {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

impl Drop for Child {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

fn main() {
    let p1 = Parent(1, Child(11), Child(12));
    {
        let p2 = Parent(2, Child(21), Child(22));
        println!("(a) p1: {:?}, p2: {:?}", p1, p2);
        // (a) p1: Parent(1, Child(11), Child(12)), p2: Parent(2, Child(21), Child(22))
        // Dropping Parent(2, Child(21), Child(22))
        // Dropping Child(21)
        // Dropping Child(22)
    }
    println!("(b) p1: {:?}", p1);
    // (b) p1: Parent(1, Child(11), Child(12))
    let p3 = Parent(3, Child(31), Child(31));
    println!("(c) p1: {:?}, p3: {:?}", p1, p3);
    // (c) p1: Parent(1, Child(11), Child(12)), p3: Parent(3, Child(31), Child(31))
    // Dropping Parent(3, Child(31), Child(31))
    // Dropping Child(31)
    // Dropping Child(31)
    // Dropping Parent(1, Child(11), Child(12))
    // Dropping Child(11)
    // Dropping Child(12)
}
