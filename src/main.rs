fn main() {
    let v1: Vec<u8> = vec![b'h', b'e', b'l', b'l', b'o'];
    let v2: Vec<u16> = v1.iter().map(|&n| n as u16).collect::<Vec<u16>>();
    println!("{:?}", v2) // [104, 101, 108, 108, 111]
}
