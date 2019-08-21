use ring::hmac;

fn main() {
    let base32_encoded_string = "LKSDJLFKLWKEJFLKWEJWEKJ";
    let a: Vec<u8> = base32::decode(
        base32::Alphabet::RFC4648 { padding: false },
        base32_encoded_string,
    )
    .unwrap();

    let b: &[u8] = &a;

    let c = hmac::Key::new(hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY, b);
    let d: hmac::Tag = hmac::sign(&c, &data());
    let e = d.as_ref();
    let offset = (e[19] & 0xf) as usize;
    let binary = (e[offset + 0] as u64 & 0xff) << 24
        | (e[offset + 1] as u64) << 16
        | (e[offset + 2] as u64) << 8
        | (e[offset + 3] as u64) << 0;
    let x = binary % 1000000;

    let y = format!("{:06}", x);

    println!("{}", y);
}

fn data() -> [u8; 8] {
    let d: std::time::SystemTime = std::time::SystemTime::now();
    let e = d.duration_since(std::time::UNIX_EPOCH).unwrap();
    let f = e.as_secs();
    let g: u64 = f / 30;
    g.to_be_bytes()
}
