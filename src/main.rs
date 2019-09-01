use std::convert::From;
use std::net::IpAddr;

fn main() {
    assert_eq!(usize::from(true), 1);
    assert_eq!(usize::from(false), 0);

    let xs: [u8; 4] = [192, 168, 0, 1];
    let addr: IpAddr = IpAddr::from(xs);

    println!("{:?}", addr); // V4(192.168.0.1)
}
