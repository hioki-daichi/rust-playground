fn main() {
    let s = "foo\nbar\nbaz";
    let mut lines = s.lines();
    assert_eq!(lines.next(), Some("foo"));
    assert_eq!(lines.next(), Some("bar"));
    assert_eq!(lines.next(), Some("baz"));
}
