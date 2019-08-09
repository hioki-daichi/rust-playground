fn main() {
    let a = "aあbい";
    assert_eq!(a.get(0..1), Some("a"));
    assert_eq!(a.get(1..3), None);
    assert_eq!(a.get(1..4), Some("あ"));
    assert_eq!(a.get(4..5), Some("b"));
    assert_eq!(a.get(5..8), Some("い"));
    assert_eq!(a.get(5..9), None);
}
