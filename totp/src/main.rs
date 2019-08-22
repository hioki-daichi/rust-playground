fn main() {
    let secret = "LKSDJLFKLWKEJFLKWEJWEKJ";

    let alphabet = base32::Alphabet::RFC4648 { padding: false };
    let decoded_secret: Vec<u8> = base32::decode(alphabet, secret).unwrap();

    let algorithm = ring::hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY;
    let key = ring::hmac::Key::new(algorithm, &decoded_secret);
    let data = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        / 30)
        .to_be_bytes();
    let mac_value = ring::hmac::sign(&key, &data);

    println!("{:?}", decoded_secret);
    println!("{:?}", key);
    println!("{:?}", mac_value);
}
