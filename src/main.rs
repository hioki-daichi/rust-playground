use lazy_static::lazy_static;
use std::collections::HashSet;
use std::error::Error;
use std::sync::RwLock;

// Lazy Static クレートの lazy_static! マクロを使って static 変数 WORDS を定義している。
// Rust の素の static 変数ではコンパイル時の定数でしか初期化できず関数呼び出しなどはできない。
// Rust の素の static 変数同様デストラクタ (drop メソッド) を呼ばない仕様になっている。
lazy_static! {
    pub static ref WORDS: RwLock<HashSet<&'static str>> = {
        let words = ["foo", "bar"].iter().cloned().collect();
        RwLock::new(words)
    };
}

fn stringify(x: impl ToString) -> String {
    x.to_string()
}

fn main() -> Result<(), Box<dyn Error>> {
    {
        let words = WORDS.read()?; // read ロックを取得している。

        assert!(words.contains("foo"));
        assert!(words.contains("bar"));
    } // words がスコープを外れるため read ロックが解除される。

    // read ロックが解除されたため write ロックを取得できる。
    // main スレッドで "baz" を追加している。
    WORDS.write()?.insert("baz");

    // 別スレッドで "qux" を追加している。
    std::thread::spawn(|| {
        WORDS
            .write()
            .map(|mut words| words.insert("qux"))
            .map_err(stringify)
    })
    .join()
    .expect("Thread error")?;

    // main スレッドで追加した "baz" も見えるし、
    assert!(WORDS.read()?.contains("baz"));
    // 別スレッドで追加した "qux" も見える。
    assert!(WORDS.read()?.contains("qux"));

    Ok(())
}
