fn main() {
    // "0..3" is an Iterator that generates: 0, 1, and 2
    let mut sequence = 0..3;

    // "next" returns the next value in the sequence
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
}