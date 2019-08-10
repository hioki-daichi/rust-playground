use std::collections::HashMap;

fn main() {
    let mut m1 = HashMap::new();

    m1.insert("a", 1);
    m1.insert("b", 2);

    // 要素数は 2
    assert_eq!(m1.len(), 2);

    // キーに対応する値を取り出す
    assert_eq!(m1.get("a"), Some(&1));
    assert_eq!(m1.get("b"), Some(&2));
    assert_eq!(m1.get("c"), None);

    // "d" が存在するならその値の参照を得る。
    // 存在しないなら "d" に対して 0 を登録してから参照を返す。
    let d = m1.entry("d").or_insert(0);
    assert_eq!(d, &0);
    *d = 1;
    assert_eq!(m1.get("d"), Some(&1));
}
