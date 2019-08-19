use std::cell::RefCell;

struct A {
    s: RefCell<String>,
}

fn main() {
    let a = A {
        s: RefCell::new("Daichi".to_string()),
    };

    let ra = &a;

    // 可変の参照を取っている
    ra.s.borrow_mut().push_str(" Hioki");

    assert_eq!(*ra.s.borrow_mut(), "Daichi Hioki".to_string());

    {
        // 不変の参照を取っている
        let ras = a.s.borrow();

        assert_eq!(*ras, "Daichi Hioki".to_string());

        // さらに a.s.borrow_mut(); で可変の参照を取ろうとすると panic する。
        // thread 'main' panicked at 'already borrowed: BorrowMutError', src/libcore/result.rs:1084:5
        //
        // RefCell は実行時に貸し出しの数をカウントして借用規則に従っているかをチェックしてくれる。

        // try_borrow_mut() なら借用の成否が Result で返る
        assert!(a.s.try_borrow_mut().is_err())
    } // ras はここでスコープを抜ける

    assert!(a.s.try_borrow_mut().is_ok())
}
