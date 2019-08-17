#[derive(Debug)]
struct ToyVec<T> {
    elements: Box<[T]>,
    len: usize, // ベクタの長さ (現在の要素数)
}

impl<T: Default> ToyVec<T> {
    fn new() -> Self {
        Self::with_capacity(0)
    }

    // 第1引数が &mut self なので ToyVec 構造体の内容を変更することがわかる
    // 第2引数が T 型なので所有権がこのメソッドへムーブすることがわかる
    fn push(&mut self, element: T) {
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
    fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            Some(&self.elements[index])
        } else {
            None
        }
    }

    // fn get_or<'a>(&'a mut self, index: usize, default: &'a T) -> &'a T {
    //     match self.get(index) {
    //         Some(v) => v,
    //         None => default,
    //     }
    // }

    // fn pop(&mut self) -> Option<T> {
    //     if self.len == 0 {
    //         None
    //     } else {
    //         self.len -= 1;
    //         let elem = std::mem::replace(&mut self.elements[self.len], Default::default());
    //         Some(elem)
    //     }
    // }

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

fn main() {
    let mut v = ToyVec::new();

    v.push("Java Finch".to_string()); // 桜文鳥
    v.push("Budgerigar".to_string()); // セキセイインコ

    // この時点では、スタック領域に v と e のためのスペースが用意されている。
    // v は ToyVec の構造体が格納されていて、e はまだ初期化されていない。
    //
    // スタック領域では ToyVec<String> の構造体が 3 行占めていて、
    //
    // - 上 2 行が Box<[String]> 型の elements フィールド
    //     - スライスを実現するデータ構造
    //     - 以下からなるファットポインタ (ファットポインタとは、データへのポインタといくつかの追加情報を含む構造体のこと)
    //         - スライスの先頭要素を指すポインタ
    //         - スライスの要素数
    // - 下 1 行が len フィールド
    //     - ベクタの要素数で、この時点では 2 になっている。
    let e = v.get(1);
    assert_eq!(e, Some(&"Budgerigar".to_string()));
}
