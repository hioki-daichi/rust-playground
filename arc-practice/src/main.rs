use std::cell::RefCell;
use std::rc::Rc;

struct Node {
    name: &'static str,
    neighbors: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    fn new(name: &'static str) -> Rc<Self> {
        Rc::new(Self {
            name,
            neighbors: RefCell::new(Vec::new()),
        })
    }
}

// 回収時に name を出力
impl Drop for Node {
    fn drop(&mut self) {
        println!("{:?}", self.name);
    }
}

fn main() {
    let node1 = Node::new("node1");
    let node2 = Node::new("node2");
    let node3 = Node::new("node3");

    // 1 -> 2
    node1.neighbors.borrow_mut().push(node2.clone());

    // 2 -> 1
    node2.neighbors.borrow_mut().push(node1.clone());

    // 3 -> 1
    node3.neighbors.borrow_mut().push(node1.clone());
} // 1 <-> 2 で循環参照しているため "node3" しか出力されない
