#[derive(Debug)]
struct Point(f64, f64); 

type Complex = (f64, f64);

fn main() {
    let origin = Point(0.0, 0.0);
    let imaginary: Complex = (0.0, 1.0);

    println!("struct: {:#?}", origin);
    println!("type: {:#?}", imaginary);
}