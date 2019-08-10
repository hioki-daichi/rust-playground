fn main() {
    let f1: f32 = 0.1234e+3;

    println!("{}", f1); // 123.4

    // 型キャストはデータ変換を伴う
    let i1 = f1 as i32;
    println!("{}", i1); // 123

    // transmute はデータ変換を伴わない。
    // このようにビット列が変換先の型にとって無意味になったとしてもコンパイルエラーにはならない
    let i2: i32 = unsafe { std::mem::transmute(f1) };
    println!("{}", i2); // 1123470541
}
