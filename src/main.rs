// 外部クレートの Struct: A をラップした Struct: AWrapper を定義する。
struct AWrapper(A);

// AWrapper は自身で定義した型なのでトレイトを実装できる。
impl A for AWrapper {}

fn main() {
    // A トレイトのメソッドが呼べる。
    AWrapper(A::new()).method();
}
