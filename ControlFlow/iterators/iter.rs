fn main() {
    let a = [1, 2, 3, 4];
    let a_iter = a.iter();
    for i in a_iter {
        println!("for {0}", i);
    }
}