use std::rc::Rc;

pub struct Stack<T>(Option<Rc<(T, Stack<T>)>>);

impl<T> Clone for Stack<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self(None)
    }

    pub fn push(self, x: T) -> Self {
        Self(Some(Rc::new((x, self))))
    }

    pub fn peek(&self) -> Option<&T> {
        if let Some(rc) = &self.0 {
            Some(&rc.0)
        } else {
            None
        }
    }
}

impl<T: Clone> Stack<T> {
    pub fn pop(self) -> (Self, Option<T>) {
        if let Some(rc) = self.0 {
            let (head, tail) = Rc::try_unwrap(rc).unwrap_or_else(|rc| (*rc).clone());
            (tail, Some(head))
        } else {
            (Self(None), None)
        }
    }
}

fn main() {
    let s: Stack<i32> = Stack::new();
    assert_eq!(s.peek(), None);

    let s = s.push(42);
    assert_eq!(s.peek(), Some(&42));

    let (s, head) = s.pop();
    assert_eq!(head, Some(42));
    assert_eq!(s.peek(), None);
}
