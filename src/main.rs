use std::collections::HashSet;
use std::error::Error;
use std::sync::{Arc, RwLock};

// read() や write() で発生する PoisonError を String に変換する便利な関数
fn stringify(a: impl ToString) -> String {
    a.to_string()
}

fn main() -> Result<(), Box<dyn Error>> {
    let hs: HashSet<&'static str> = ["foo", "bar"].iter().cloned().collect();

    // RwLock で包むことで HashSet を可変にして、
    // Arc で包むことでスレッド間で共有できるようにしている
    let hs: Arc<RwLock<HashSet<&'static str>>> = Arc::new(RwLock::new(hs));

    {
        // read ロックを取得している。
        //
        // Arc は内包するデータ型 (ここでは RwLock) への Deref を実装しているため、
        // メソッドレシーバの型強制により RwLock のメソッドを直に呼べる。
        //
        // ロックが取得できない時は取得できるまで現在のスレッドがブロックされる。
        //
        // ロックが取得できると Ok(ガード) を返すため ? 演算子でアンラップしている。
        //
        let hs2 = hs.read().map_err(stringify)?;

        assert!(hs2.contains("foo"));
        assert!(hs2.contains("bar"));
    } // hs2 がスコープをはずれロックが解除される

    // write ロックを取得している。
    // Ok(ガード) を ? 演算子でアンラップした後、メソッドレシーバの型強制によって
    // ガードから HashSet<&'static str> の insert() を呼んでいる。
    hs.write().map_err(stringify)?.insert("baz");

    // 複製した Arc ポインタを spawn で起動した別スレッドに渡すことで HashSet を共有している
    let hs3: Arc<RwLock<HashSet<&'static str>>> = Arc::clone(&hs);
    std::thread::spawn(move || {
        let _guard = hs3.write();
        panic!();
    })
    .join()
    .expect_err(""); // expect_err は Err を期待するため Ok の場合に panic する。

    let _hs4 = hs.read().expect("Cannot acquire read lock");
    // thread '<unnamed>' panicked at 'explicit panic', src/main.rs:42:9
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
    // thread 'main' panicked at 'Cannot acquire read lock: "PoisonError { inner: .. }"', src/libcore/result.rs:1084:5

    // main スレッドで追加した要素は見えるし、
    assert!(hs.read().map_err(stringify)?.contains("baz"));

    // 別スレッドで追加した要素も見える。
    assert!(hs.read().map_err(stringify)?.contains("qux"));

    Ok(())
}
