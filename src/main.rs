fn f<T: std::borrow::Borrow<str>>(s: T) {
    assert_eq!("Hello", s.borrow());
}

fn g<T: AsRef<str>>(s: T) {
    assert_eq!("Hello", s.as_ref());
}

fn main() {
    f("Hello");
    f("Hello".to_string());

    g("Hello");
    g("Hello".to_string());
}
