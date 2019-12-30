use std::rc::Rc;
// use std::sync::Arc;
use std::thread;

fn main() {
    // let rc = Arc::new(42);
    let rc = Rc::new(42);

    println!("{:?}", rc);

    let thread = thread::spawn(move || eprintln!("value = {}", rc));

    thread.join().unwrap();
}
