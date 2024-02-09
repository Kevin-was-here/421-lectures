fn main() {
    let x = 101;
    let borrowed = &x;
    let boxed = Box::new(x);

    println!("borrowed is a pointer {:?} = {:p}", borrowed as *const i32, &x);
    println!("Box points to a copy: {:?}", Box::into_raw(boxed));
    // This is also a leak because boxed has now been consumed
}