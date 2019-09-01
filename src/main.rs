fn main() {
    println!("{:?}", std::mem::size_of::<&String>()); // 8
    println!("{:?}", std::mem::size_of::<&str>()); // 16
    println!("{:?}", std::mem::size_of::<&i32>()); // 8
    println!("{:?}", std::mem::size_of::<&[i32]>()); // 16
    println!("{:?}", std::mem::size_of::<&[i32; 4]>()); // 8
    println!("{:?}", std::mem::size_of::<&Vec<i32>>()); // 8
}
