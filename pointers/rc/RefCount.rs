use std::rc::Rc;

//ref counting pointers
fn main() {
    let a = Rc::new(String::from("Shared string"));
    
    // new owner, wrapper around the box pointer
    println!("initial ref count {}", Rc::strong_count(&a));

    {
        let b = Rc::clone(&a);

        println!("a's ref count {}", Rc::strong_count(&a));
        println!("b's ref count {}", Rc::strong_count(&b));

        // Like box, Rc has Deref and Drop traits implemented
        assert_eq!(a.len(), b.len());
        println!("Just checking: {}", b);
    }

    println!("ref count after b is dropped {}", Rc::strong_count(&a));
} // ref count goes to zero when a goes out of scope