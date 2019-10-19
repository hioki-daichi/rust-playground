trait MyIterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

impl<I: MyIterator> MyIterator for &mut I {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        (*self).next()
    }
}

struct SetOnDrop<'a, T> {
    borrow: &'a mut T,
    value: Option<T>,
}

impl<T> Drop for SetOnDrop<'_, T> {
    fn drop(&mut self) {
        if let Some(x) = self.value.take() {
            *self.borrow = x;
        }
    }
}

fn main() {}
