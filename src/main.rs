fn main() {
    assert_eq!(
        Some(300).and_then(|n| if n > 255 { Some(255) } else { Some(n) }),
        Some(255)
    );
}
