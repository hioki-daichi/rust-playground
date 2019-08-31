struct Foo {}

trait ATrait {
    const AAA: u32;
}

impl ATrait for Foo {
    const AAA: u32 = 2;
}

fn main() {
    println!("{}", Foo::AAA); // 2
}
