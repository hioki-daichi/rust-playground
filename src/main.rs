fn main() {
    // このタプルはスタックに置かれる。
    let t1: (i32, String) = (3, "birds".to_string());

    // Box ポインタを作ることでタプルがヒープに移動する。
    let mut b1 = Box::new(t1);

    // * で参照外しすることで参照先にアクセスする。
    (*b1).0 += 1;

    assert_eq!(*b1, (4, "birds".to_string()));
}
