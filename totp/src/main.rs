use ring::hmac;

fn main() {
    // base32 encode された文字列
    let base32_encoded_string = "JBSWY3DPFQQEEQKTIUZTEICXN5ZGYZBB";

    let alphabet = base32::Alphabet::RFC4648 { padding: false };

    let base32_decoded_string: Vec<u8> = base32::decode(alphabet, base32_encoded_string).unwrap();

    let algorithm = hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY;

    let key = hmac::Key::new(algorithm, &base32_decoded_string);

    let data = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        / 30)
        .to_be_bytes();

    let tag = hmac::sign(&key, &data);
    let mac = tag.as_ref();
    let offset = (mac[19] & 0xf) as usize;
    let a = ((mac[offset + 0] & 0x7f) as u64) << 24;
    let b = ((mac[offset + 1] & 0xff) as u64) << 16;
    let c = ((mac[offset + 2] & 0xff) as u64) << 8;
    let d = ((mac[offset + 3] & 0xff) as u64) << 0;
    let e = a + b + c + d;
    let x = format!("{:06}", e % 1000000);
    println!("{}", x);
}
