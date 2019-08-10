fn main() {
    assert_eq!("10".parse::<i32>(), Ok(10));

    let r = "a".parse::<i32>();
    assert!(r.is_err());
    println!("{:?}", r); // Err(ParseIntError { kind: InvalidDigit })
}
