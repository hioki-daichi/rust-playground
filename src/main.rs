struct Circle {
    radius: u32,
}

// 構造体のメソッドは impl ブロックの中に書く
impl Circle {
    // 第1引数はメソッドのレシーバが渡される。
    // &self と書くことで、self という名前のイミュータブルな参照として
    // Circle構造体のインスタンスの情報を使うことができる。
    // ミュータブルな参照として使いたい場合は &mut self とすればよい。
    // 単に self とすると、メソッドのレシーバの所有権がメソッドに移動する。
    // 第1引数の型は明らかなため省略できる。
    fn diameter(&self) -> u32 {
        self.radius * 2
    }

    // これは関連関数。インスタンスではなく、構造体そのものに関連付く関数。
    // 第1引数は self にしない。
    fn small_circle() -> Circle {
        Circle { radius: 1 }
    }
}

fn main() {
    // 関連関数の呼び出し時には :: を使って Circle:: のように指定する。
    let circle: Circle = Circle::small_circle();

    // Circle::diameter メソッドの第1引数は &self (不変の参照) であるため
    // 本来なら (&circle).diameter() という形で実行しなければならないが、
    // メソッドの呼び出しについてはコンパイラが自動で変換してくれるため気にしなくてよい。
    assert_eq!(circle.diameter(), 6);
}
