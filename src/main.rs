fn apply_fn<F>(f: &F, ch: char)
where
    F: Fn(char) -> bool, // ここでの F は Fn を実装している
{
    assert!(f(ch));
}

fn apply_fn_mut<F>(f: &mut F, ch: char)
where
    F: FnMut(char) -> bool, // ここでの F は FnMut を実装している
{
    assert!(f(ch));
}

fn apply_fn_once<F>(f: F, ch: char)
where
    F: FnOnce(char) -> bool, // ここでの F は FnOnce を実装している
{
    assert!(f(ch));
}

fn main() {
    let s1: &str = "read-only";

    // 環境から読むだけのクロージャ。
    // find(&self) は &str を読むだけ。
    let mut lookup = |ch| s1.find(ch).is_some();

    apply_fn(&lookup, 'r');
    apply_fn_mut(&mut lookup, 'r');
    apply_fn_once(lookup, 'r');

    // 環境に取り込まれた文字列 (&str 型) は変更されていない。
    assert_eq!(s1, "read-only");

    let mut s2: String = "append".to_string();
    let mut modify = |ch| {
        s2.push(ch); // push(&mut self, char) は String を可変の参照経由で変更する
        true
    };

    // コンパイルエラー
    // apply_fn(&modify, 'e'); // expected a closure that implements the `Fn` trait, but this closure only implements `FnMut`    the requirement to implement `Fn` derives from here

    apply_fn_mut(&mut modify, '!');
    apply_fn_once(modify, '?');

    assert_eq!(s2, "append!?");

    let s3 = "be converted".to_string();
    let mut consume = |ch| {
        let bytes = s3.into_bytes();
        bytes.contains(&(ch as u8))
    };

    // apply_fn(&consume, 'b'); // expected a closure that implements the `Fn` trait, but this closure only implements `FnOnce`    the requirement to implement `Fn` derives from here
    // apply_fn_mut(&mut consume, 'c'); // expected a closure that implements the `FnMut` trait, but this closure only implements `FnOnce`    the requirement to implement `FnMut` derives from here
    apply_fn_once(consume, 'd');

    // コンパイルエラー
    // s3 はムーブ済み
    // assert_eq!(s3, "error"); // borrow of moved value: `s3`    value borrowed here after move
}
