fn f(p: Box<[i32]>) -> i32 {
    p[0]
}

fn main() {
    let xs: [i32; 4] = [1, 2, 3, 4];

    let b: Box<[i32; 4]> = Box::new(xs);

    // Box<[i32; 4]> から Box<[i32]> へサイズの不定化が行われている
    assert_eq!(1, f(b));
}
