trait Init<T> {
    fn init(t: T) -> Self;
}

impl<T> Init<T> for Box<T> {
    fn init(t: T) -> Self {
        Box::new(t)
    }
}

fn main() {
    let data = Box::init("foo");
    println!("{:?}", data); // "foo"

    let data = Box::<f32>::init(0.1);
    println!("{:?}", data); // 0.1
}
