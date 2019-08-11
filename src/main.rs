fn main() {
    let mut counter = 10;

    let a = loop {
        if counter == 0 {
            break counter;
        }

        counter -= 1;
    };

    assert_eq!(a, 0);
}
