#[derive(Debug, PartialEq)]
struct UniqueValue();

fn main() {
    let a = UniqueValue();
    let b = UniqueValue();
    assert_eq!(a, b);
}
