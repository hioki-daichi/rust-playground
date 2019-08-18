mod toy_vec;

use toy_vec::ToyVec;

fn main() {
    let mut v = ToyVec::new();

    v.push("Java Finch".to_string());
    v.push("Budgerigar".to_string());

    let mut iter = v.iter();

    // コンパイルエラー
    // push は可変の参照を得ようとしているが、iter が生存しているため不変の参照が有効
    // v.push("Hill Mynah".to_string()); // cannot borrow `v` as mutable because it is also borrowed as immutable    mutable borrow occurs here

    assert_eq!(iter.next(), Some(&"Java Finch".to_string()));

    v.push("Canary".to_string());
}
