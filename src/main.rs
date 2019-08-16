struct ToyVec<T> {
    elements: Box<[T]>,
}

impl<T: Default> ToyVec<T> {
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(n: usize) -> Self {
        Self {
            elements: Self::allocate_in_heap(n),
        }
    }

    fn allocate_in_heap(n: usize) -> Box<[T]> {
        std::iter::repeat_with(Default::default)
            .take(n)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    pub fn push(&mut self, e: T) {
        self.elements[self.elements.len()] = e;
    }
}

fn main() {
    let mut v: ToyVec<i32> = ToyVec::new();

    // 実行時エラー
    v.push(0); // thread 'main' panicked at 'index out of bounds: the len is 0 but the index is 0', src/main.rs:24:9
}
