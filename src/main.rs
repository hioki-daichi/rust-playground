fn main() {
    let v1 = vec![3, 4, 5];

    // Vec<i32> 型からスライス &[i32] 型へ型強制されることによって
    // スライスに備わった first(&self) メソッドが使用できる
    assert_eq!(Some(&3), v1.first());

    // もし型強制がなかったら
    assert_eq!(Some(&3), (&v1[..]).first());
}
