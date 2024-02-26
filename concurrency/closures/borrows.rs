fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    //since the value is already borrowed/owned, it is captured in a read-only manner
    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();

    //capturing the value in a mutable manner since it is already borrowed/owned
    let mut borrows_mutably = || list.push(4);

    //this would be a immutable borrow to print it and this is not allowed.
    // println!("both a mutable and immutable borrow: {:?}", list);
    borrows_mutably();

    println!("After mutable borrow is done {:?}", list);
}