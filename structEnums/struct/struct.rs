struct Rectangle {
    width: u32,
    depth: u32,
}

fn main() {
    let square = Rectangle {
        width: 16,
        depth: 16,
    };

    println!("dimension: {}x{}", square.width, square.depth);
}