fn main() {
    for i in (0..=3).rev() {
        print!("{i} ");
    }
    println!("Blast-off!");

    for i in (0..).skip(10).take(4) {
        print!("{} ", i);
    }
}