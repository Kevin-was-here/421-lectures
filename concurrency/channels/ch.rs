use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};


//channels are for message passing between threads
fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();  // so we can have two senders
    thread::spawn(move || { 
        let start = Instant::now();
        for _ in 1..5 {
            let msg = format!(" <t1: elapsed {:?}> ", start.elapsed());
            tx1.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    //can allowed for multiple senders
    thread::spawn(move || {
        let vals = vec![
            String::from(" #t2# "),
            String::from(" #t2 again# "),
            String::from(" #t2 more# "),
            String::from(" #t2 last# "),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("{}", received);
    }
}