fn main() {
    let p1: Box<i32> = Box::new(10);

    // 生ポインタ *mut i32 型に型キャストはできない
    // let p2 = p1 as *mut i32; // non-primitive cast: `std::boxed::Box<i32>` as `*mut i32`    note: an `as` expression can only be used to convert between primitive types. Consider using the `From` trait

    println!("{:p}", p1); // 0x7fa42dc029d0

    // Box ポインタと *mut ポインタはどちらも同じビット幅
    println!("{}", std::mem::size_of::<Box<i32>>()); // 8
    println!("{}", std::mem::size_of::<*mut i32>()); // 8

    // なので transmute できる
    let p3: *mut i32 = unsafe { std::mem::transmute(p1) };

    println!("{:p}", p3); // 0x7fa42dc029d0
}
