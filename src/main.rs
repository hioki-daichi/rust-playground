#[derive(Default)]
struct A {
    f0: u8,
    f1: u32, // x86_64 アーキテクチャでは i32 は 4 の倍数のアドレスにアラインされていることを要求する
    f2: u8,
}

fn main() {
    let a: A = Default::default();
    println!(
        "struct A ({} bytes) \n  \
         f0: {:p}\n  \
         f1: {:p}\n  \
         f2: {:p}\n", // {:p} で格納されているアドレスを表示
        std::mem::size_of::<A>(),
        &a.f0,
        &a.f1,
        &a.f2
    );
    // struct A (8 bytes)
    //   f0: 0x7ffee06c45f4 // 末尾 4 のため、f1 より後に定義されていることがわかる
    //   f1: 0x7ffee06c45f0 // 末尾 0 のため、f0 より先に定義されていることがわかる
    //   f2: 0x7ffee06c45f5
}
