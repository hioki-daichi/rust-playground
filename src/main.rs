#[allow(overflowing_literals)]
fn main() {
    let u: u8 = 0b11111111;
    let i: i8 = 0b11111111;

    println!("{:08b}", u >> 2); // 00111111 (論理右シフト演算)
    println!("{:08b}", i >> 2); // 11111111 (算術右シフト演算)
}
