mod toy_vec;

use toy_vec::*;

fn main() {
    let mut v = ToyVec::new();

    v.push("Java Finch".to_string()); // 桜文鳥
    v.push("Budgerigar".to_string()); // セキセイインコ

    // この時点では、スタック領域に v と e のためのスペースが用意されている。
    // v は ToyVec の構造体が格納されていて、e はまだ初期化されていない。
    //
    // スタック領域では ToyVec<String> の構造体が 3 行占めていて、
    //
    // - 上 2 行が Box<[String]> 型の elements フィールド
    //     - スライスを実現するデータ構造
    //     - 以下からなるファットポインタ (ファットポインタとは、データへのポインタといくつかの追加情報を含む構造体のこと)
    //         - スライスの先頭要素を指すポインタ
    //         - スライスの要素数
    // - 下 1 行が len フィールド
    //     - ベクタの要素数で、この時点では 2 になっている。
    let e = v.get(1);
    assert_eq!(e, Some(&"Budgerigar".to_string()));

    let e2 = v.pop();
    assert_eq!(e2, Some("Budgerigar".to_string()));

    let foo = "foo".to_string();
    let e3 = v.get_or(2, &foo);
    assert_eq!(e3, &foo);
}
