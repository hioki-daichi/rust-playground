fn main() {
    let s = "かか\u{3099}く";
    println!("{}", s); // かがく

    let mut iter = s.chars();
    assert_eq!(iter.next(), Some('か'));
    assert_eq!(iter.next(), Some('か'));
    assert_eq!(iter.next(), Some('\u{3099}'));
    assert_eq!(iter.next(), Some('く'));
}
