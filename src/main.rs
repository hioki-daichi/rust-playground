struct X();

trait A {
    fn f(self) -> Self
    where
        Self: Sized,
    {
        self
    }
}

trait B: A {
    fn g(self);
}

impl A for X {}

impl B for X {
    fn g(self) {}
}

fn main() {
    X().f();
}
