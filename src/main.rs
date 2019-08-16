fn main() {
    let mut a = vec![1, 2, 3];
    println!("{:?}", a); // [1, 2, 3]

    let b = vec![4, 5, 6];
    println!("{:?}", b); // [4, 5, 6]

    let c = std::mem::replace(&mut a, b);
    println!("{:?}", c); // [1, 2, 3]

    println!("{:?}", a); // [4, 5, 6]
}
