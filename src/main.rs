use std::fmt::Display;

fn main() {
    let mut v: Vec<&dyn Display> = vec![];
    v.push(&true);
    v.push(&1i32);
}
