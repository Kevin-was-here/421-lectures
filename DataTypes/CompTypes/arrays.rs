fn main() {
    let a = [1, 2, 3, 4];
    let specific: [i16; 4] = a;
    // initialize all elements to the same value
    let repeat = [0u8; 4];

    println!("index as expected: {}", specific[0]);
    println!("dbg pretty print: {:?}", repeat);
}

    