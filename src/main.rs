// ジェネリクスにライフタイム境界を付ける場合
// 'static ライフタイムを持つ任意の型を引数にとる
fn take_static<T: 'static>(_x: T) {}

fn main() {
    let s1 = "42";
    let s2 = 42.to_string();

    take_static(s1); // OK

    // コンパイルエラー
    take_static(&s2); // `s2` does not live long enough    borrowed value does not live long enough

    take_static(s2); // OK
}
