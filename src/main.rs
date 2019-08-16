#[derive(Debug)]
struct ToyVec<T> {
    elements: Box<[T]>,
    len: usize,
}

impl<T: Default> ToyVec<T> {
    pub fn new() -> Self {
        Self {
            elements: Self::allocate_in_heap(0),
            len: 0,
        }
    }

    fn allocate_in_heap(n: usize) -> Box<[T]> {
        std::iter::repeat_with(Default::default)
            .take(n)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    pub fn push(&mut self, e: T) {
        if self.len == self.capacity() {
            self.grow()
        }
        self.elements[self.len] = e;
        self.len += 1;
    }

    fn capacity(&self) -> usize {
        self.elements.len()
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
}

fn main() {
    let mut v: ToyVec<i32> = ToyVec::new();
    println!("{:?}", v); // ToyVec { elements: [], len: 0 }

    v.push(1);
    println!("{:?}", v); // ToyVec { elements: [1], len: 1 }

    v.push(2);
    println!("{:?}", v); // ToyVec { elements: [1, 2], len: 2 }
}
