use ring::hmac;

fn main() {
    let base32_encoded_string = "YOIZZNEW62LZXIC4A7WFRFB5WYEMCDYCE4HBM6E5INKBHA2F5OHJAEVGWWS3A7TN";
    // println!("{}", base32_encoded_string);

    let secret = base32::decode(
        base32::Alphabet::RFC4648 { padding: false },
        &base32_encoded_string,
    )
    .unwrap();

    // println!("{:?}", secret);

    let key = hmac::Key::new(hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY, &secret);

    let data = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        / 30)
        .to_be_bytes();

    let tag = hmac::sign(&key, &data);
    let value = tag.as_ref();
    let offset = (value[19] & 0xf) as usize;
    // println!("{:?}", value);
    // println!("{:?}", offset);
    let a = ((value[offset + 0] & 0x7f) as u64) << 24;
    let b = ((value[offset + 1] & 0xff) as u64) << 16;
    let c = ((value[offset + 2] & 0xff) as u64) << 8;
    let d = ((value[offset + 3] & 0xff) as u64) << 0;
    // println!("{:?} {:?} {:?} {:?}", a, b, c, d);
    let sum = a + b + c + d;
    // println!("{:?}", sum);
    // println!("{:?}", sum % 1000_000);
    let res = format!("{:06}", sum % 1000_000);
    println!("{:?}", res);
}
