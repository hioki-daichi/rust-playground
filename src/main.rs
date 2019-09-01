use std::ffi::OsStr;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

fn hello_to_file(path: impl AsRef<Path>) -> io::Result<()> {
    let mut buf = File::create(path.as_ref())?;
    buf.write(b"hello")?;
    Ok(())
}

fn main() {
    hello_to_file("foo.txt").unwrap();
    hello_to_file("bar.txt".to_string()).unwrap();
    hello_to_file(OsStr::new("baz.txt")).unwrap();
    hello_to_file(Path::new("qux.txt")).unwrap();
}
