use std::sync::Arc;
use std::thread;

fn main() {
    let rc = Arc::new(42);

    println!("{:?}", rc);

    let thread = thread::spawn(move || eprintln!("value = {}", rc));

    thread.join().unwrap();
}
