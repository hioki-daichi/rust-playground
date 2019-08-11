fn main() {
    let mut one = 1;
    // move キーワードを使うことで、変数を貸すのではなくコピーさせる。
    // 整数は Copy トレイトを実装しているため所有権を移転するのではなくコピーされる。
    let plus_one = move |x| x + one;
    one += 1;
    println!("10 + 1 = {}", plus_one(10)); // 10 + 1 = 11
    println!("one = {}", one); // 2
}
