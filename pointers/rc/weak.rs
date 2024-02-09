use std::rc::Rc;


//weak pointers are strong pointers that dont have ownership

fn main() {
    let a = Rc::new("shared string".to_string());
    let c = {
        let b = Rc::clone(&a);

        //downgraded to weak pointer
        let d = Rc::downgrade(&b); 

        //this is where we get the weak and strong counts 
        println!("b's counts strong {}, weak {}", Rc::strong_count(&b), Rc::weak_count(&b));
        d
    };

    println!("a's counts strong {}, weak {}", Rc::strong_count(&a), Rc::weak_count(&a));

    if let Some(d) = c.upgrade() {
       println!("can use the upgraded weak pointer: {}", d);
    }

    drop(a);

    match c.upgrade() {
        Some(d) => println!("still here! {}", d),
        None => println!("all is lost!"),
    }
}