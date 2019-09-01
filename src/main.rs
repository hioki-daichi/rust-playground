use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let data = b"some bytes";

    let mut pos = 0;
    let mut buffer = File::create("foo.txt")?;

    while pos < data.len() {
        let buf = &data[pos..pos + 1];
        let bytes_written = buffer.write(buf)?;
        pos += bytes_written;
    }
    Ok(())
}
