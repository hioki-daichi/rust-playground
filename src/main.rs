use std::cell::RefCell;
use std::collections::HashSet;

fn main() {
    // 標準ライブラリの thread_local! マクロで TLS に変数 SS を作成している
    thread_local!(
        static SS: RefCell<HashSet<&'static str>> = {
            // >> let ss: HashSet<&str> = ["foo", "bar"].iter().cloned().collect();
            // >> ss
            // {"bar", "foo"}
            let ss = ["foo", "bar"].iter().cloned().collect();
            RefCell::new(ss)
        }
    );

    // TLS に置いた値にアクセスするには with を使う。
    SS.with(|ss| {
        // ss の型は &RefCell<HashSet<&'static str>>

        assert!(ss.borrow().contains("foo"));
        assert!(ss.borrow().contains("bar"));

        // "baz" を追加する。
        assert!(!ss.borrow().contains("baz"));
        ss.borrow_mut().insert("baz");
        assert!(ss.borrow().contains("baz"));
    });

    // 別スレッドを起動する。
    std::thread::spawn(|| {
        SS.with(|ss| {
            assert!(ss.borrow().contains("foo"));
            assert!(ss.borrow().contains("bar"));

            // main スレッドで追加した "baz" はこちらのスレッドには存在しない。
            assert!(!ss.borrow().contains("baz"));

            // "qux" を追加する。
            assert!(!ss.borrow().contains("qux"));
            ss.borrow_mut().insert("qux");
            assert!(ss.borrow().contains("qux"));
        });
    })
    .join()
    .expect("Thread error");

    // 再び main スレッド
    SS.with(|ss| {
        // 以前 main スレッドで追加した "baz" がいる。
        assert!(ss.borrow().contains("baz"));
        // 別スレッドで追加した "qux" はいない。
        assert!(!ss.borrow().contains("qux"));
    })
}
