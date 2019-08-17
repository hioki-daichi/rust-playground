#[derive(Debug)]
pub struct ToyVec<T> {
    elements: Box<[T]>,
    pub len: usize, // ベクタの長さ (現在の要素数)
}

impl<T: Default> ToyVec<T> {
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn push(&mut self, element: T) {
        if self.len == self.capacity() {
            self.grow();
        }

        self.elements[self.len] = element;
        self.len += 1;
    }

    pub fn get(&self, n: usize) -> Option<&T> {
        if n < self.len {
            Some(&self.elements[n])
        } else {
            None
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            let elem = std::mem::replace(&mut self.elements[self.len], Default::default());
            Some(elem)
        }
    }

    fn with_capacity(n: usize) -> Self {
        Self {
            elements: Self::allocate_in_heap(n),
            len: n,
        }
    }

    fn allocate_in_heap(n: usize) -> Box<[T]> {
        std::iter::repeat_with(Default::default)
            .take(n)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn grow(&mut self) {
        if self.capacity() == 0 {
            self.elements = Self::allocate_in_heap(1);
        } else {
            let new_elements = Self::allocate_in_heap(self.capacity() * 2);
            let old_elements = std::mem::replace(&mut self.elements, new_elements);
            for (i, element) in old_elements.into_vec().into_iter().enumerate() {
                self.elements[i] = element;
            }
        }
    }

    pub fn capacity(&self) -> usize {
        self.elements.len()
    }
}
