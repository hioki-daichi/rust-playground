use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let data = b"some bytes";

    let mut pos = 0;
    let mut buffer = File::create("foo.txt")?;

    while pos < data.len() {
        println!("{:?}", buffer); // File { fd: 3, path: "/Users/daichi/.ghq/github.com/hioki-daichi/rust-playground/foo.txt", read: false, write: true }
        let buf = &data[pos..];
        println!("{:?}", buf); // [115, 111, 109, 101, 32, 98, 121, 116, 101, 115]
        let bytes_written = buffer.write(buf)?;
        println!("{:?}", bytes_written); // 10
        pos += bytes_written;
    }
    Ok(())
}
