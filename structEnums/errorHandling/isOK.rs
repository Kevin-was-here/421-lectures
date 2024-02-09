use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut handle = match File::open("hello.txt") {
        Ok(fd) => fd,
        Err(e) => {
            println!("Open failed: {}", e);
            return Err(e)
        },
    };

    let mut data = vec![];
    handle.read_to_end(&mut data)?;

    println!("read: {:?}", data);

    Ok(())
}