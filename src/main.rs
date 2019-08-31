trait Foo<T> {
    fn new(t: T) -> Self;
}
fn some_fn_foo1<S, T: Foo<S>>(t: T) {}
fn some_fn_foo2<S, T: Foo<u32>>(t: T) {}

struct Baz;

impl Foo<i32> for Baz {
    fn new(_t: i32) -> Self {
        Baz {}
    }
}

impl Foo<char> for Baz {
    fn new(_t: char) -> Self {
        Baz {}
    }
}

fn main() {}
