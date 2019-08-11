fn main() {
    let mut counter = 10;

    while counter != 0 {
        counter -= 1;
    }

    assert_eq!(counter, 0);
}
