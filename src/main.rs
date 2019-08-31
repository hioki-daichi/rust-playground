trait Overload {
    fn call(&self) -> &'static str;
}

impl Overload for i32 {
    fn call(&self) -> &'static str {
        "i32"
    }
}

impl Overload for str {
    fn call(&self) -> &'static str {
        "str"
    }
}

fn main() {
    assert_eq!(1i32.call(), "i32");
    assert_eq!("str".call(), "str");

    // Rust のメソッド呼び出しはただの糖衣構文なので以下のようにも書ける。
    assert_eq!(Overload::call(&1i32), "i32");
    assert_eq!(Overload::call("str"), "str");
}
