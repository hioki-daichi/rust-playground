#[derive(Debug)]
struct ToyVec<T> {
    elements: Box<[T]>,
    len: usize,
}

impl<T: Default> ToyVec<T> {
    pub fn new() -> Self {
        Self {
            elements: vec![].into_boxed_slice(),
            len: 0,
        }
    }
}

fn main() {
    let v: ToyVec<i32> = ToyVec::new();
    println!("{:?}", v);
}
