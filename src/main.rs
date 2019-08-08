fn main() {
    let c1 = 'A';
    let c1_ptr: &char = &c1; // &char 型。不変の参照。
    assert_eq!(*c1_ptr, 'A');

    let mut n1 = 0;
    let n1_ptr: &mut i32 = &mut n1; // &mut i32 型。可変の参照。
    assert_eq!(*n1_ptr, 0);

    *n1_ptr = 1_000;
    assert_eq!(*n1_ptr, 1_000);
}
