#[allow(unused_imports)]
// rand は外部のクレートなので絶対指定。
// rand クレートの prelude モジュールの中のすべてのアイテムを使っている。
use rand::prelude::*;

mod network {
    pub fn ping() {
        println!("Ping");
    }
}

fn main() {
    // ルートモジュールからの相対指定
    network::ping(); // Ping

    // 明示的にクレートのルートモジュールを表す crate を書いてもよい
    crate::network::ping(); // Ping

    // self は現在地のモジュールを指すが、現在地はルートモジュールなので次のようにも書ける
    self::network::ping(); // Ping
}
