fn main() {
    let o1 = Some("Apple");

    // 複合的な型 (e.g. 列挙型, 構造体, タプル, 参照)
    // の値からその中の値を取り出すことを「分配束縛」と呼ぶ。
    let s1 = match o1 {
        // o1 がバリアント Option::Some の場合に、
        // 変数 s2 をバリアントに関連付けられている値 ("Apple") に束縛している。
        Some(s2) => String::from("Hi, ") + s2,
        None => String::from("Nothing"),
    };

    println!("{}", s1); // Hi, Apple
}
