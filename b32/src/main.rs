fn main() {
    let data = "YOIZZNEW62LZXIC4A7WFRFB5WYEMCDYCE4HBM6E5INKBHA2F5OHJAEVGWWS3A7TN";

    let a: Vec<u8> = data
        .as_bytes()
        .into_iter()
        .map(|a| format!("{:0b}", a))
        .collect::<Vec<String>>()
        .join("")
        .as_bytes()
        .chunks(5)
        .map(|a| {
            *b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567"
                .get(
                    isize::from_str_radix(&String::from_utf8(a.to_vec()).unwrap(), 2).unwrap()
                        as usize,
                )
                .unwrap()
        })
        .collect();

    println!("{:?}", a);
}

// "YOIZZNEW62LZXIC4A7WFRFB5WYEMCDYCE4HBM6E5INKBHA2F5OHJAEVGWWS3A7TN"
// [195, 145, 156, 180, 150, 246, 151, 155, 160, 92, 7, 236, 88, 148, 61, 182, 8, 193, 15, 2, 39, 14, 22, 120, 157, 67, 84, 19, 131, 69, 235, 142, 144, 18, 166, 181, 165, 176, 126, 109]

// fn decode(data: &str) -> Vec<u8> {
//     let data = data.as_bytes();
//     for chunk in data.chunks(8) {
//         let buf = {
//             let mut buf = [0u8; 8];
//             for (i, &c) in chunk.iter().enumerate() {
//
//             }
//         }
//     }
// }

// #[test]
// fn test() {
//     assert_eq!(encode(&[0xF8, 0x3E, 0x7F, 0x83, 0xE7]), "7A7H7A7H");
// }
//
// #[test]
// fn test_decode() {
//     assert_eq!(decode("7A7H7A7H").unwrap(), [0xF8, 0x3E, 0x7F, 0x83, 0xE7]);
// }
