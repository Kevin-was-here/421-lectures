fn print_max<T: std::cmp::PartialOrd + std::fmt::Display>(x: T, y: T) {
    if x >= y {
        println!("{x} >= {y}");
    } else {
        println!("{y} > {x}");
    }
}

fn main() {
    print_max(123, 321);
    print_max(3.14, 1.414);
}