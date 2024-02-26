use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn main() {
    let counter = Mutex::new(0);
    let handle = thread::spawn(move || {
        for i in 1..10 {
            {
                let mut val = counter.lock().unwrap();
                *val = i;
            }
            println!("number {:?} from the spawned thread!", counter);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();
    //println!("final number {:?} from main", counter);
}