fn positive(val: i32) -> bool {
    let result;  // declaration without initialization 😱
    if val >= 0 {
        result = true;
    } else {
        result = false;
    }  // no semi-colon when used as a statement
    result
}

fn main() {
    println!("I'm positive about this: {}", positive(-3));
}