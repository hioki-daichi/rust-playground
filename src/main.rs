// std::io 名前空間を io としてインポートしている。
// こうすることで、フルパスだと std::io::stdin() という形で参照しなければならない関数が
// io::stdin() と書けるようになる。
use std::io;

// std::io::Write トレイトを使う
use std::io::Write;

// エントリポイントとなる関数
fn main() {
    // これは文。文とは () を返すプログラム要素。文の末尾には ; を付ける。
    // 文には大きく分けて宣言文と式文がある。
    // 宣言文にはアイテム宣言(use 宣言や関数定義など)と let 文とがある。
    // なので以下は文の中でも宣言文で、宣言文の中でも let 文。
    let mut year = String::new();

    // 以下は式文(のはず)。式文とは、式の末尾に ; を付けることで式を文に変換したもの。
    print!("Please input a year to check if it is a leap year: ");

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut year).unwrap();

    let year = year.trim().parse::<u32>().unwrap();

    // 以下は末尾の ; を省略している式文。
    // true 節と false 節の戻り値が () の場合には if 式の末尾に ; がなくても式文と同様に取り扱う特
    // 例があるためだそうな ( See: https://exoskeleton.dev/proglang/rust/rust-6.md#if式 )
    if is_leap_year(year) {
        println!("{} is a leap year!", year);
    } else {
        println!("{} is not a leap year.", year);
    }
}

// `fn` は関数を定義するためのキーワード。
// 関数定義から { 本体 } を除いた部分を関数のシグネチャと呼ぶ。
// 例えば以下の関数のシグネチャはは `fn is_leap_year(year: u32) -> bool`
fn is_leap_year(year: u32) -> bool {
    // これは式。式とは () 以外の値を返すプログラム要素。
    year % 4 == 0 && !(year % 100 == 0 && year % 400 != 0)
}
