fn main() {
    let a: [u8; 4] = [0x61, 0xe3, 0x81, 0x82];
    assert_eq!(std::str::from_utf8(&a), Ok("aあ"));
}
