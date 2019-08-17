fn f<T: Default>(n: usize) -> Box<[T]> {
    std::iter::repeat_with(Default::default)
        .take(n)
        .collect::<Vec<_>>()
        .into_boxed_slice()
}

fn main() {
    let a: Box<[i32]> = f(3);
    assert_eq!(a[0..=2], [0, 0, 0]);
}
