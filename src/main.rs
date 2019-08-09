fn main() {
    let a = ['a', 'b', 'c', 'd'];
    assert_eq!(&a[1..2], ['b']);
    assert_eq!(&a[1..=2], ['b', 'c']);
    assert_eq!(&a[3..], ['d']);
    assert_eq!(&a[..3], ['a', 'b', 'c']);
    assert_eq!(&a[..], ['a', 'b', 'c', 'd']);
}
