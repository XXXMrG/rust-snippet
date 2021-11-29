use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn concurrent_run_test() {
        concurrent_run();
    }

    #[test]
    fn channel_example_test() {
        channel_example();
    }

    #[test]
    fn channel_example_multiple_message_test() {
        channel_example_multiple_message();
    }

    #[test]
    fn mutex_example_test() {
        assert_eq!(mutex_example(), 10);
    }
}

pub fn concurrent_run() {
    let handler = thread::spawn(|| {
        for i in 1..10 {
            println!("print from spwan thread {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..6 {
        println!("print from main thread {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    handler.join().unwrap();
}

pub fn channel_example() {
    // multiple producer, single consumer
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();

        // val has been move to main thread, so can't visit val here!
    });

    // will hang on main thread, until get sender's message
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

pub fn channel_example_multiple_message() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("1: hi"),
            String::from("1: from"),
            String::from("1: the"),
            String::from("1: thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("2: more"),
            String::from("2: message"),
            String::from("2: for"),
            String::from("2: you"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    println!("main thread end!");
}

pub fn mutex_example() -> i32 {
    // atomically reference counted
    let counter = Arc::new(Mutex::new(0));
    let mut handlers = vec![];

    for _ in 0..10 {
        let counter = counter.clone();
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });

        handlers.push(handle);
    }

    for handle in handlers {
        handle.join().unwrap();
    }

    let res = *counter.lock().unwrap();

    println!("Result is {}", res);

    return res;
}
