use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

pub fn concurrency() {
    thread::spawn(|| {
        for i in 1..5 {
            // print i, then sleep thread for 2 milliseconds
            println!("Secondary Thread Prints {}", i);
            thread::sleep(Duration::from_millis(2));
        }
    });

    for i in 1..5 {
        // print i, then sleep thread for 2 milliseconds
        println!("Main Thread Prints {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}

pub fn concurrency_with_join() {
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("Secondary Thread Prints {}", i);
            thread::sleep(Duration::from_millis(2));
        }
    });

    for i in 1..5 {
        println!("Main Thread Prints {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

pub fn concurrency_with_move() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Secondary Thread Prints {:?}", v);
    });

    handle.join().unwrap();
}

pub fn concurrency_with_channel() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Hello from the secondary thread");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Main Thread Prints {}", received);
}

pub fn share_state_concurrency() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // clone the reference to counter
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // lock the mutex to acquire the lock
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    // wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}



#[cfg(test)]
mod concurrency_tests {
    use super::*;

    #[test]
    fn test_concurrency() {
        concurrency();
    }

    #[test]
    fn test_concurrency_with_join() {
        concurrency_with_join();
    }

    #[test]
    fn test_concurrency_with_move() {
        concurrency_with_move();
    }

    #[test]
    fn test_concurrency_with_channel() {
        concurrency_with_channel();
    }

    #[test]
    fn test_share_state_concurrency() {
        share_state_concurrency();
    }
}