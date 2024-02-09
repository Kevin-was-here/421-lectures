fn main() {
    let x: i32 = 101;
    let boxed = Box::new(x);
    let borrowed = &x;

    println!("Display is handled {} = {}", borrowed, boxed);

    assert_eq!(x, *boxed);
    assert_eq!(x, *borrowed);
    // boxed is dropped here
}