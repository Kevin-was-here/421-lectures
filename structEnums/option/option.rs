pub enum Option<T> {
    None,
    Some(T),
}

// The Rust prelude automatically "uses" std::Option::{Some, None}
fn main() {
    let present_u32 = Some(5);
    let absent_u32: Option<u32> = None;

    assert_eq!(present_u32.is_some(), true);
    assert_eq!(absent_u32.is_none(), true);

    assert_eq!(present_u32.is_some_and(|val| val == 5), true);

    if let Some(val) = present_u32 {
        println!("present {val}");
    }

    match absent_u32 {
        Some(val) => println!("absent {val}"),
        None => println!("absent is absent"),
    }

    println!("unwrapped {}", absent_u32.unwrap_or_default());
}