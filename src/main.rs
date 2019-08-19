use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Child(u32);

fn main() {
    // Rc::new 関連関数に Child 構造体の値を与えることで Rc ポインタを作っている。
    // Box<T> 同様、リソースはヒープ領域に移動する。
    let mut rc1: Rc<Child> = Rc::new(Child(100));

    // Rc::strong_count で参照カウント (共同所有者の数) が得られる。
    assert_eq!(Rc::strong_count(&rc1), 1);

    {
        assert_eq!(rc1.0, 100);

        // Rc::get_mut は参照カウントが 1 の場合は可変の参照が得られる。
        if let Some(child) = Rc::get_mut(&mut rc1) {
            println!("Some!!"); // Some!!
            child.0 += 1; // 1 を足す。
        }

        assert_eq!(rc1.0, 101); // 1 が足された。

        // Rc::clone 関連関数でポインタを複製している。
        // 複数されるのはポインタだけでリソースは複製されない。
        // Rc::clone で共同所有者を作ると参照カウントが増える。
        assert_eq!(Rc::strong_count(&rc1), 1);
        let _rc2 = Rc::clone(&rc1);
        assert_eq!(Rc::strong_count(&rc1), 2);

        // 参照カウントが 2 なので Rc::get_mut の結果は None になる。
        if let None = Rc::get_mut(&mut rc1) {
            println!("None!!") // None!!
        }
    }

    // _rc2 がスコープを抜けたため参照カウントが減った。
    assert_eq!(Rc::strong_count(&rc1), 1);

    // Rc::downgrade で Weak ポインタが得られる。
    // Weak ポインタはリソースの共同所有権を持たないポインタで、
    // 参照カウント (strong_count) とは別に weak_count としてカウントされる。
    // が、リソースは weak_count とは関係なく strong_count が 0 になった時に開放される。
    assert_eq!(Rc::weak_count(&rc1), 0);
    let weak: Weak<Child> = Rc::downgrade(&rc1);
    assert_eq!(Rc::weak_count(&rc1), 1);

    // Weak ポインタは参照カウントが増えない。
    assert_eq!(Rc::strong_count(&rc1), 1);

    println!("{:?}", weak); // (Weak)

    // Weak を Rc にアップグレードすると...
    if let Some(rc3) = weak.upgrade() {
        // 参照カウントが増える。
        assert_eq!(Rc::strong_count(&rc1), 2);

        // Child 値にアクセスできる。
        assert_eq!(rc3.0, 101);
    }
    // 抜けると参照カウントは減る。
    assert_eq!(Rc::strong_count(&rc1), 1);

    // rc1 を drop する。(スコープを抜けたのと同じ)
    // 参照カウントは 0 になり、Child は破棄される。
    std::mem::drop(rc1);

    // コンパイルエラー
    // println!("{}", Rc::strong_count(&rc1)); // borrow of moved value: `rc1`    value borrowed here after move

    // drop によって Child 値がすでに破棄されているため weak.upgrade() しても None が返る。
    println!("weak.upgrade(): {:?}", weak.upgrade()); // None
}
