struct ToyVec<T> {
    elements: Box<[T]>,
}

impl<T: Default> ToyVec<T> {
    pub fn new(n: usize) -> Self {
        Self {
            elements: std::iter::repeat_with(Default::default)
                .take(n)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }
}

fn main() {
    let v: ToyVec<i32> = ToyVec::new(2);

    println!("{:?}", v.elements.get(0)); // Some(0)
    println!("{:?}", v.elements.get(1)); // Some(0)
    println!("{:?}", v.elements.get(2)); // None
}
