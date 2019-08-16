struct ToyVec<T> {
    elements: Box<[T]>,
    len: usize,
}

impl<T: Default> ToyVec<T> {
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    fn with_capacity(capacity: usize) -> Self {
        Self {
            elements: Self::allocate_in_heap(capacity),
            len: capacity,
        }
    }

    fn allocate_in_heap(size: usize) -> Box<[T]> {
        std::iter::repeat_with(Default::default)
            .take(size)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn capacity(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, e: T) {
        if self.elements.len() == self.capacity() {
            self.grow();
        }
        self.elements[self.len] = e;
        self.len += 1;
    }

    fn grow(&mut self) {
        if self.capacity() == 0 {
            self.elements = Self::allocate_in_heap(1);
        } else {
            let new_elements = Self::allocate_in_heap(self.capacity() * 2);
            let old_elements = std::mem::replace(&mut self.elements, new_elements);
            for (i, elem) in old_elements.into_vec().into_iter().enumerate() {
                self.elements[i] = elem;
            }
        }
    }

    pub fn pop(&self) -> Option<&T> {
        self.elements.get(self.len - 1)
    }
}

fn main() {
    let mut v = ToyVec::new();
    v.push(1);
    let x = v.pop();
    println!("{:?}", x);
}
