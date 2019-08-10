fn main() {
    let a: Box<std::fmt::Debug>;

    let b: [i32; 2] = [1, 2];
    let c: Option<i32> = Some(1);

    // ある型が Debug トレイトを実装していればその型からトレイトオブジェクトへ型強制できる
    a = Box::new(b); // Box<[i32; 2]>  -> Box<Debug>
    a = Box::new(c); // Box<Some<i32>> -> Box<Debug>
}
