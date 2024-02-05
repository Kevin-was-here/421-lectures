fn main() {
    let tup: (i32, f64, u8) = (100, 100.0, 100);

    // This is called destructuring
    let (i, f, b) = tup;

    println!("The float value is {} = {}", f, tup.1);
}