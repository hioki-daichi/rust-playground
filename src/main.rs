fn f(xs: &[i32]) -> Option<i32> {
    let s0 = xs.get(0)?;
    let s3 = xs.get(2)?;
    Some(s0 + s3)
}

fn main() {
    assert_eq!(f(&[1, 2, 3]), Some(4));
    assert_eq!(f(&[1, 2]), None);
}
