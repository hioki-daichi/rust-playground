fn main() {
    let mut v1: Vec<i32> = vec![0, 1, 2, 3];

    // 余分なメモリを確保する様子
    println!("capacity: {}", v1.capacity()); // capacity: 4
    v1.push(4);
    println!("capacity: {}", v1.capacity()); // capacity: 8

    // Box<[i32]> に変換すると Vec の shrink_to_fit() が呼ばれて余分なメモリを持たなくなる様子
    println!("capacity: {}", v1.into_boxed_slice().into_vec().capacity()); // capacity: 5
}
