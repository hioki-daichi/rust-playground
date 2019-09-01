fn main() {
    let a = [1, 2, 3, 4];
    assert_eq!(a.iter().fold(0, |acc, x| acc + x), 10);
    assert_eq!(a.iter().fold(1, |acc, x| acc * x), 24);
}
