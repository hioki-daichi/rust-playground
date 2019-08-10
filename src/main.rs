fn f(slice: &mut [usize]) {
    // ポインタの弱体化 (mutability の除去)
    // &mut [usize] -> &[usize]
    let len = g(slice);
    slice[0] = len;
}

fn g(slice: &[usize]) -> usize {
    slice.len()
}

fn main() {
    let mut v: Vec<usize> = vec![0; 10];
    f(&mut v); // Deref による型強制: &mut Vec<usize> -> &mut [usize]
    assert_eq!(10, v[0]);
}
