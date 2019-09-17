pub struct ToyVec<T> {
    elements: Box<[T]>,
    len: usize,
}

impl<T: Default> ToyVec<T> {
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            elements: Self::allocate_in_heap(capacity),
            len: 0,
        }
    }

    fn allocate_in_heap(size: usize) -> Box<[T]> {
        std::iter::repeat_with(Default::default)
            .take(size)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.elements.len()
    }

    pub fn push(&mut self, element: T) {
        if self.len == self.capacity() {
            if self.capacity() == 0 {
                self.elements = Self::allocate_in_heap(1);
            } else {
                // 既存の全要素を新しい Box<[T]> へムーブしている
                let new_elements = Self::allocate_in_heap(self.capacity() * 2); // 現在の2倍のキャパシティをもつ領域を確保し直す
                let old_elements = std::mem::replace(&mut self.elements, new_elements);
                for (i, elem) in old_elements.into_vec().into_iter().enumerate() {
                    self.elements[i] = elem;
                }
            }
        }
        self.elements[self.len] = element;
        self.len += 1;
    }

    // 要素を借用できる
    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len() {
            Some(&self.elements[index])
        } else {
            None
        }
    }

    pub fn get_or<'a>(&'a self, index: usize, default: &'a T) -> &'a T {
        match self.get(index) {
            Some(v) => v,
            None => default,
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            // let elem = self.elements[self.len];
            let elem = std::mem::replace(&mut self.elements[self.len], Default::default()); // 代わりの値と交換
            Some(elem)
        }
    }

    // 本来ならライフタイム指定子を書く必要はないが、
    // self と戻り値 Iter のライフタイムが `'vec` で同じであることを明示的に書くことで
    // Iter が生存している間は ToyVec が借用されていることを説明している。
    pub fn iter<'vec>(&'vec self) -> Iter<'vec, T> {
        Iter {
            elements: &self.elements,
            len: self.len,
            pos: 0,
        }
    }
}

pub struct Iter<'vec, T> {
    elements: &'vec Box<[T]>,
    len: usize,
    pos: usize,
}

impl<'vec, T> Iterator for Iter<'vec, T> {
    type Item = &'vec T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.len {
            None
        } else {
            let res = Some(&self.elements[self.pos]);
            self.pos += 1;
            res
        }
    }
}

fn main() {
    // let mut v = ToyVec::new();
    // println!("{:?}", v);
    // v.push("foo".to_string());
    // println!("{:?}", v);
    // v.push("bar".to_string());
    // println!("{:?}", v);
    // v.push("baz".to_string());
    // println!("{:?}", v);
    // v.push("qux".to_string());
    // println!("{:?}", v);
    // v.push("quux".to_string());
    // println!("{:?}", v);
    // v.push("corge".to_string());
    // println!("{:?}", v);
    // v.push("grault".to_string());
    // println!("{:?}", v);
    // v.push("garply".to_string());
    // println!("{:?}", v);
    // v.push("waldo".to_string());
    // println!("{:?}", v);
    // v.push("fred".to_string());
    // println!("{:?}", v);
    // v.push("plugh".to_string());
    // println!("{:?}", v);
    // v.push("xyzzy".to_string());
    // println!("{:?}", v);
    // v.push("thud".to_string());
    // println!("{:?}", v);
    // // len: 0, capacity: 0, elements: []
    // // len: 1, capacity: 1, elements: ["foo"]
    // // len: 2, capacity: 2, elements: ["foo", "bar"]
    // // len: 3, capacity: 4, elements: ["foo", "bar", "baz", ""]
    // // len: 4, capacity: 4, elements: ["foo", "bar", "baz", "qux"]
    // // len: 5, capacity: 8, elements: ["foo", "bar", "baz", "qux", "quux", "", "", ""]
    // // len: 6, capacity: 8, elements: ["foo", "bar", "baz", "qux", "quux", "corge", "", ""]
    // // len: 7, capacity: 8, elements: ["foo", "bar", "baz", "qux", "quux", "corge", "grault", ""]
    // // len: 8, capacity: 8, elements: ["foo", "bar", "baz", "qux", "quux", "corge", "grault", "garply"]
    // // len: 9, capacity: 16, elements: ["foo", "bar", "baz", "qux", "quux", "corge", "grault", "garply", "waldo", "", "", "", "", "", "", ""]
    // // len: 10, capacity: 16, elements: ["foo", "bar", "baz", "qux", "quux", "corge", "grault", "garply", "waldo", "fred", "", "", "", "", "", ""]
    // // len: 11, capacity: 16, elements: ["foo", "bar", "baz", "qux", "quux", "corge", "grault", "garply", "waldo", "fred", "plugh", "", "", "", "", ""]
    // // len: 12, capacity: 16, elements: ["foo", "bar", "baz", "qux", "quux", "corge", "grault", "garply", "waldo", "fred", "plugh", "xyzzy", "", "", "", ""]
    // // len: 13, capacity: 16, elements: ["foo", "bar", "baz", "qux", "quux", "corge", "grault", "garply", "waldo", "fred", "plugh", "xyzzy", "thud", "", "", ""]

    let mut v = ToyVec::new();
    v.push("Java Finch".to_string());
    v.push("Budgerigar".to_string());
    let mut iter = v.iter();
    // v.push("Hill Mynah".to_string()); // コンパイルエラー
    assert_eq!(iter.next(), Some(&"Java Finch".to_string()));
    v.push("Canary".to_string());
}

// capacity(&self) も都度しりたいため #[derive(Debug)] せずに自分で書く
impl<T: std::fmt::Debug + Default> std::fmt::Debug for ToyVec<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(
            f,
            "len: {}, capacity: {}, elements: {:?}",
            self.len,
            self.capacity(), // capacity も表示したい
            self.elements,
        )?;
        Ok(())
    }
}
