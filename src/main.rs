static mut V: Option<Vec<i32>> = None;

fn main() {
    unsafe {
        V = Some(vec![1]);
        println!("{:?}", V); // Some([1])
    }
}
