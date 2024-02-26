use std::thread;

fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let thread_output = thread::spawn(move || { //move the list into the thread
        //works without move, but then the list is borrowed and can't be used in main
        //move is a shallow copy, clone is a deep copy
        println!("I own this: {:?}", list);
        list.push(4);
        list
    });

    //immutealbe reference to list since it's still owned by the thread
    //println!("You can't print list here {:?}", list);
    
    if let Ok(list) = thread_output.join() {
        //thread is done so list is back in main's scope
        println!("And now main owns it again {:?}", list);
    }
}