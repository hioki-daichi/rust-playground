fn main() -> io::Result<()> {
    let s = "foo";
    let t = *s; // the size for values of type `str` cannot be known at compilation time
}
