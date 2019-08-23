use ring::hmac;
fn main() {
    let b32_encoded_string = "YOIZZNEW62LZXIC4A7WFRFB5WYEMCDYCE4HBM6E5INKBHA2F5OHJAEVGWWS3A7TN";
    let alphabet = base32::Alphabet::RFC4648 { padding: false };
    let secret = base32::decode(alphabet, b32_encoded_string).unwrap();
    // println!("{:?}", secret);

    let msg = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        / 30)
        .to_be_bytes();

    let key = hmac::Key::new(hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY, &secret);
    let result = hmac::sign(&key, &msg);
    let value = result.as_ref();
    // println!("{:?}", value);
    let offset = (value[19] & 0xf) as usize;
    let a = ((value[offset + 0] & 0x7f) as u64) << 24;
    let b = (value[offset + 1] as u64) << 16;
    let c = (value[offset + 2] as u64) << 8;
    let d = value[offset + 3] as u64;
    println!("{:?} {:?} {:?} {:?}", a, b, c, d);
    let sum = a + b + c + d;
    println!("{:?}", sum);
    println!("{:?}", sum % 1000_000);
    println!("{:?}", format!("{:06}", sum % 1000_000));
}
