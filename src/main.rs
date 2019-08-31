fn foo(s: impl Into<String>) {
    println!("{:?}", s.into());
}

fn main() {
    foo("A"); // "A"
    foo("B".to_string()); // "B"
}
