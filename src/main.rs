fn f1(n: &mut usize, str: &str, slice: &[i32]) {
    *n = str.len() + slice.len()
}

fn main() {
    let mut b: Box<usize> = Box::new(0);
    let s: String = String::from("foo");
    let v: Vec<i32> = vec![1];

    // Deref による型強制が起こる
    //
    //     &mut Box<usize> -> &mut usize
    //     &String         -> &str
    //     &Vec<i32>       -> &[i32]
    //
    f1(&mut b, &s, &v);

    assert_eq!(4, *b);
}
