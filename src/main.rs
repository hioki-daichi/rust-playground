fn main() {
    let mut foo: String = "foo".to_string();
    let bar: &str = "bar";

    foo.push_str(bar); // String 型の文字列に &str 型の文字列を追加

    assert_eq!(foo, "foobar");

    let baz: String = String::from("baz");

    foo.push_str(&baz); // & を付けると型強制という仕組みによって &str へ変換される

    assert_eq!(foo, "foobarbaz");
}
