struct A {
    s: String,
}

fn main() {
    let a = A {
        s: "daichi".to_string(),
    };

    let ra = &a;

    // コンパイラエラー
    // 不変の参照経由でフィールドを変更しようとしている
    ra.s.push('a'); // cannot borrow `ra.s` as mutable, as it is behind a `&` reference    `ra` is a `&` reference, so the data it refers to cannot be borrowed as mutable
}
