use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
    time::Duration,
};

pub fn learn_threads() {
    let counter = Arc::new(Mutex::new(0));

    let mut all_handles = vec![];

    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        });
        all_handles.push(handle);
    }

    for h in all_handles {
        h.join().unwrap();
    }

    println!("{:?}", *counter.lock().unwrap());
}
