fn f(base32_encoded_str: &str) -> String {
    // println!("{}", base32_encoded_str);
    let base32_decoded_str = base32::decode(
        base32::Alphabet::RFC4648 { padding: false },
        base32_encoded_str,
    )
    .unwrap();
    // println!("{:?}", base32_decoded_str);

    use ring::hmac;
    let key = hmac::Key::new(hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY, &base32_decoded_str);
    let duration = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    let counter = (duration.as_secs() / 30).to_be_bytes();
    let tag = hmac::sign(&key, &counter);
    let value = tag.as_ref();
    let offset = (value[19] & 0xf) as usize;
    // println!("{}", offset);
    let a = ((value[offset + 0] & 0x7f) as u64) << 24;
    let b = ((value[offset + 1] & 0xff) as u64) << 16;
    let c = ((value[offset + 2] & 0xff) as u64) << 8;
    let d = ((value[offset + 3] & 0xff) as u64) << 0;
    // println!("{}", a);
    // println!("{}", b);
    // println!("{}", c);
    // println!("{}", d);
    let sum = a + b + c + d;
    // println!("{}", sum);
    let x = sum % 1000000;
    // println!("{}", x);
    let y = format!("{:06}", x);
    // println!("{}", y);
    y
}

fn main() {
    let base32_encoded_str = "JBSWY3DPFQQEEQKTIUZTEICXN5ZGYZBB";
    println!("{}", f(&base32_encoded_str));
}
