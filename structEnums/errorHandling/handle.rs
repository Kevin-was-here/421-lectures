use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut handle = File::open("hello.txt").unwrap();

    let mut data = vec![];
    handle.read_to_end(&mut data);

    println!("read: {:?}", data);

    Ok(())
}