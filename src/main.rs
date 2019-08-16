#[derive(Debug)]
struct ToyVec<T> {
    elements: Box<[T]>,
    len: usize,
}

impl<T: Default> ToyVec<T> {
    pub fn new(n: usize) -> Self {
        Self {
            elements: std::iter::repeat_with(Default::default)
                .take(n)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            len: n,
        }
    }
}

fn main() {
    let v: ToyVec<i32> = ToyVec::new(2);
    println!("{:?}", v);
}
