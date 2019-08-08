fn double(n: i32) -> i32 {
    n + n
}

fn abs(n: i32) -> i32 {
    if n >= 0 {
        n
    } else {
        -n
    }
}

fn main() {
    let mut f: fn(i32) -> i32 = double; // `fn(i32) -> i32` が関数ポインタ型
    assert_eq!(f(-42), -84);

    f = abs; // abs も double 同様型が `fn(i32) -> i32` で合うため代入できる
    assert_eq!(f(-42), 42);

    // 関数ポインタはポインタの一種なのでサイズは usize 型と同じ
    assert_eq!(std::mem::size_of_val(&f), std::mem::size_of::<usize>());
}
