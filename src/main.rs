fn main() {
    // static 変数。'static スコープを持つ。
    static I0: i32 = 42;

    // 'static スコープを持つ str への可変の参照
    let mut s0: &'static str;

    // 文字列リテラルへの参照。データは静的領域にある。
    let s1: &str = "42";

    // String 型。データはヒープ領域にある。
    let s2 = 42.to_string();

    // 文字列リテラルへの参照は 'static ライフタイムを持つため束縛可能。
    s0 = s1;

    // コンパイルエラー
    // String 型から &'static str は作れない。
    // s0 = &s2; // `s2` does not live long enough    borrowed value does not live long enough
}
