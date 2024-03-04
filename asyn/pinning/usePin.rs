extern crate pin_utils;

use std::marker::PhantomPinned;
use pin_utils::pin_mut;

#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
    _pin_me: PhantomPinned, //does not take any space
}

fn main() {
    let mut test1 = Test::new("test1");
    test1.init();
    pin_mut!(test1);
    let mut test2 = Test::new("test2");
    test2.init();
    pin_mut!(test2);

    println!("before swap: &test1 {:p}, size {}", &test1, std::mem::size_of::<Test>());

    println!("test1: a: {} {:p}, b: {} {:p}", test1.a(), &test1.a, test1.b(), test1.b);
    println!("test2: a: {} {:p}, b: {} {:p}", test2.a(), &test2.a, test2.b(), test2.b);
    
    std::mem::swap(&mut test1, &mut test2);

    println!("after swap &test2 {:p}, size {}", &test2, std::mem::size_of::<Test>());

    //test1.a = "Surprise!".to_string();
    println!("test1: a: {} {:p}, b: {} {:p}", test1.a(), &test1.a, test1.b(), test1.b);
    println!("test2: a: {} {:p}, b: {} {:p}", test2.a(), &test2.a, test2.b(), test2.b);
}