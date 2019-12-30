use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));

    let thread = thread::spawn({
        let counter = counter.clone();
        move || {
            for _ in 0..100000 {
                let mut counter = counter.lock().unwrap();

                if *counter % 2 == 0 {
                    *counter += 1
                }
            }
        }
    });

    for _ in 0..100000 {
        let mut counter = counter.lock().unwrap();
        if *counter % 2 == 1 {
            *counter += 1
        }
    }

    thread.join().unwrap();

    let counter = *counter.lock().unwrap();
    println!("{:?}", counter);
}
