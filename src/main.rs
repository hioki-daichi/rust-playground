fn main() {
    let mut s1 = String::from("Type coercion ");
    let s2 = String::from("is actually easy.");

    // push_str() のシグネチャは push_str(self: &mut String, s: &str)
    // 型強制によって s1 は String 型から &mut String 型へ、
    //                s2 は String 型から &str 型へ変換される。
    s1.push_str(&s2);

    // もし型強制がなかったら
    (&mut s1).push_str(s2.as_str());
}
