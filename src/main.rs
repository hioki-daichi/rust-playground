use std::fs::File;
use std::io;
use std::os::unix::prelude::FileExt;

fn main() -> io::Result<()> {
    let mut buf = [0u8; 8];
    let file = File::open("foo.txt")?; // "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
    let num_bytes_read = file.read_at(&mut buf, 2)?;
    println!("read {} bytes: {:?}", num_bytes_read, buf); // read 8 bytes: [67, 68, 69, 70, 71, 72, 73, 74]
    Ok(())
}
