mod toy_vec;

use toy_vec::*;

fn main() {
    let mut v: ToyVec<usize> = ToyVec::new();
    v.push(100);

    // 不変の借用が取得されている
    let e = v.get(0);

    // コンパイルエラー
    // ベクタを変更しようとしているが、後に不変の借用 e にアクセスしているため借用規則に反している
    v.push(200);

    assert_eq!(e, Some(&100));
}
