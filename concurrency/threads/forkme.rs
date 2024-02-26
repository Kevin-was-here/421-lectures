use std::thread;
use std::time::Duration;

fn main() {

    //spawning chilren
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("thread count {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("main count {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    //join() waits for the child to finish
    //the childs actually will mess up printf so they need to be locked properly

    //hahahah i see what you did there
    //panic! = quit()
    handle.join().expect("child panicked!");
}