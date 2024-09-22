use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

#[allow(dead_code)]
fn without_mutex(){
    let mut counter: i32 = 0; // unsafe variable for threading
    let mut list: Vec<JoinHandle<()>> = vec![];

    for _ in 1..10{
        let handle: JoinHandle<()> = thread::spawn(move || {
            counter += 1 // Concurrent access
        });
        list.push(handle);
    }

    for i in list{
        i.join().unwrap();
    }

    println!("Counter: {}", counter)
}

// With Mutex
fn with_mutex(){
    let counter = Arc::new(Mutex::new(0));  // Use Arc and Mutex for locking
    let mut list = vec![];

    for _ in 1..10{
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap(); // lock for exclusive access
            *num += 1; // safe counter value
        });

        list.push(handle);
    }

    for i in list{
        i.join().unwrap()
    }

    println!("Counter: {}", *counter.lock().unwrap());
}

fn main() {
    // without_mutex()
    with_mutex()
}
