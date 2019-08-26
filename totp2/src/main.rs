fn main() {
    let base32_encoded_string = "YOIZZNEW62LZXIC4A7WFRFB5WYEMCDYCE4HBM6E5INKBHA2F5OHJAEVGWWS3A7TN";
    // println!("{:?}", base32_encoded_string);
    let shared_secret = base32::decode(
        base32::Alphabet::RFC4648 { padding: false },
        base32_encoded_string,
    )
    .unwrap();
    // println!("{:?}", shared_secret);
    // ring
    let key = ring::hmac::Key::new(ring::hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY, &shared_secret);
    let data = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        / 30)
        .to_be_bytes();
    let tag = ring::hmac::sign(&key, &data);
    let mac = tag.as_ref();
    let offset = (mac[19] & 0xf) as usize;
    let sum = ((mac[offset + 0] & 0x7f) as u32) << 24
        | (mac[offset + 1] as u32) << 16
        | (mac[offset + 2] as u32) << 8
        | (mac[offset + 3] as u32);
    println!("{:06}", sum % 1000_000);
}
