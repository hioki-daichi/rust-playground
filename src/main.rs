trait MyIterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

impl<'a, I: MyIterator> MyIterator for &'a mut I {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        (*self).next()
    }
}

fn main() {}
