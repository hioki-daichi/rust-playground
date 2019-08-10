fn main() {
    let a: Vec<u8> = vec![1];

    // a.first() の際に、レシーバの型強制が起きている。
    // a は Vec<u8> 型だが、この型に first() は定義されていないため、
    // 以下のように暗黙的に型強制されていく。
    //
    //     1. Deref による型強制: Vec<u8> -> [u8]
    //     2. レシーバの参照化:   [u8]    -> &[u8]
    //
    assert_eq!(a.first(), Some(&1u8));
}
