fn main() {
    let mut counter = Some(0);

    // while let 式。パターンマッチに成功した場合にループ本体が実行される。
    while let Some(i) = counter {
        if i == 10 {
            counter = None;
        } else {
            println!("{}", i);
            counter = Some(i + 1);
        }
    }
    // => 0
    // => 1
    // => 2
    // => 3
    // => 4
    // => 5
    // => 6
    // => 7
    // => 8
    // => 9
}
