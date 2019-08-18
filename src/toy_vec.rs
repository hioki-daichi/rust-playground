#[derive(Debug)]
pub struct ToyVec<T> {
    elements: Box<[T]>,
    len: usize, // ベクタの長さ (現在の要素数)
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

impl<T: Default> ToyVec<T> {
    pub fn iter<'vec>(&'vec self) -> Iter<'vec, T> {
        Iter {
            elements: &self.elements,
            len: self.len,
            pos: 0,
        }
    }

    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    // 第1引数が &mut self なので ToyVec 構造体の内容を変更することがわかる
    // 第2引数が T 型なので所有権がこのメソッドへムーブすることがわかる
    pub fn push(&mut self, element: T) {
        // 要素を追加するスペースがないなら
        if self.len == self.capacity() {
            // もっと大きい elements を確保して既存の要素を引っ越す
            self.grow();
        }

        // self.grow() で十分なキャパシティが確保できたら
        // element を Box<[T]> へムーブする
        self.elements[self.len] = element; // 所有権がムーブしている
        self.len += 1;
    }

    // 第1引数が &self なので ToyVec 構造体の内容は変更されないことがわかる
    // 第2引数は usize なので値がコピーされる
    // Option<&T> を返すため self が所有する値の不変の参照を返すことがわかる
    #[allow(dead_code)]
    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            Some(&self.elements[index])
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn get_or<'a>(&'a mut self, index: usize, default: &'a T) -> &'a T {
        self.get(index).unwrap_or(default)
    }

    // 戻り値が参照でないため所有権ごと返すことがわかる
    #[allow(dead_code)]
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
            // 現在の 2 倍の長さの Box<[T]> を作成し、
            let new_elements = Self::allocate_in_heap(self.capacity() * 2);
            // 既存の全要素を
            let old_elements = std::mem::replace(&mut self.elements, new_elements);
            // into_iter() で各要素の所有権をとるイテレータを作成している
            for (i, element) in old_elements.into_vec().into_iter().enumerate() {
                // self.elements にセットする。
                self.elements[i] = element;
            }
        }
    }

    fn capacity(&self) -> usize {
        self.elements.len()
    }
}
