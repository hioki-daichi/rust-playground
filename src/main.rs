use std::borrow::Borrow;

fn f<T: Borrow<str> + ?Sized>(a: &T) {
    let s: &str = a.borrow();
    assert_eq!("Hello", s);
}

fn main() {
    f("Hello");
    f(&"Hello".to_string());
}
