fn f(a: &str, b: &str) -> Result<i32, std::num::ParseIntError> {
    let x = a.parse::<i32>()?;
    let y = b.parse::<i32>()?;
    Ok(x + y)
}

fn main() {
    assert_eq!(f("1", "2"), Ok(3));
    assert!(f("1", "a").is_err());
}
