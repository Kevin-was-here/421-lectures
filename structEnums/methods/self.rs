struct Rectangle {
    width: u32,
    depth: u32,
}

impl Rectangle {
    // create() is an "associated function", because it is not called on an instance
    fn create(w: u32, depth: u32) -> Rectangle {
        Rectangle {
            width: w,
            depth,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.depth
    }
}

fn main() {
    let square = Rectangle::create(16, 16);

    println!("squarea: {}", square.area());
}