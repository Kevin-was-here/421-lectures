use std::convert::TryFrom;

fn main() {
    // explicit type specifications
    let float = 2.71828_f32;
    let word : i32 = 35;

    let short = float as u16;
    // only bytes can be converted to char using 'as'
    let character = word as u8 as char;

    println!("Casting: {float} -> {short}, {} -> {}", word, character);

    // From and Into are reciprocal traits
    let index = usize::try_from(word).unwrap();
    let bytebool : u8 = true.into();

    println!("index {0}, value of true {1}", index, bytebool);
}