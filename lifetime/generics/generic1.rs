fn print_max<T>(x: T, y: T) {
    if x >= y {
        println!("{x} >= {y}");
    } else {
        println!("{y} > {x}");
    }
}

fn main() {
    print_max::<u16>(123, 321);
    print_max(3.14, 1.414);
}

//will not work since the compiler cannot determine the type of the variable