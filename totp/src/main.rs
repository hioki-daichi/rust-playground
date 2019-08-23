use ring::hmac;
fn main() {
    let b32_encoded_string = "YOIZZNEW62LZXIC4A7WFRFB5WYEMCDYCE4HBM6E5INKBHA2F5OHJAEVGWWS3A7TN";
    let alphabet = base32::Alphabet::RFC4648 { padding: false };
    let secret = base32::decode(alphabet, b32_encoded_string).unwrap();
    let unixtime = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let counter = unixtime / 30;
    let msg = counter.to_be_bytes();
    let key = hmac::Key::new(hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY, &secret);
    let result = hmac::sign(&key, &msg);
    let value = result.as_ref();
    let offset = (value[19] & 0xf) as usize;
    let a0 = value[offset + 0];
    let a1 = a0 & 0x7f;
    let b1 = value[offset + 1];
    let c1 = value[offset + 2];
    let d1 = value[offset + 3];
    let a2 = ((a1 & 0x7f) as u64) << 24;
    let b2 = (b1 as u64) << 16;
    let c2 = (c1 as u64) << 8;
    let d2 = d1 as u64;
    let sum = a2 + b2 + c2 + d2;

    println!("Arg: {}", b32_encoded_string);
    println!("");
    println!(
        "Key: |{}|",
        secret
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect::<Vec<_>>()
            .join("|")
    );
    println!(
        "Msg: |{}|\n     (Unixtime: {} ‒‒(/30)‒‒> {})",
        &msg.iter()
            .map(|b| format!("{:02X}", b))
            .collect::<Vec<_>>()
            .join("|"),
        unixtime,
        counter,
    );
    println!("");
    println!(
        "MAC: |{}|",
        value
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect::<Vec<_>>()
            .join("|")
    );
    println!(
        "     {} {}{}**",
        String::from(" ").repeat(offset * 3),
        String::from("‾").repeat(11),
        String::from(" ").repeat(46 - (offset * 3)),
    );
    println!(
        "0x{:02X?} ‒‒(binary)‒‒‒> {:08b}\n     ‒‒(& 0x0F)‒‒‒> {:08b}\n     ‒‒(decimal)‒‒> {}",
        value[19],
        value[19],
        value[19] & 0xf,
        offset,
    );
    println!("");
    println!(
        "  0x{:02X?}: {:08b}\n& 0x7F: {:08b}\n‒‒‒‒‒‒‒‒‒‒‒‒‒‒‒‒\n  0x{:02X?}: {:08b}\n",
        a0, a0, 0x7f, a1, a1
    );
    println!("  0x{:02X?}: {:08b} --(<< 24)-> {:032b}", a1, a1, a2);
    println!("  0x{:02X?}: {:08b} --(<< 16)-> {:032b}", b1, b1, b2);
    println!("  0x{:02X?}: {:08b} --(<<  8)-> {:032b}", c1, c1, c2);
    println!("+ 0x{:02X?}: {:08b} --(<<  0)-> {:032b}", d1, d1, d2);
    println!("{}", String::from("‒").repeat(61));
    println!("binary: {: >53}", format!("{:032b}", sum));
    println!("decimal: {: >52}", format!("{}", sum));
    println!("                                                       ~~~~~~");
    println!("");
    println!("totp: {: >55}", format!("{:06}", sum % 1000_000));
}
