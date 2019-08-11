fn main() {
    let a = 0b11110000;
    let b = 0b01010000;
    let c = 0b00001010;

    println!("{:08b}", a & b); // 01010000
    println!("{:08b}", a & b | c); // 01011010
}
