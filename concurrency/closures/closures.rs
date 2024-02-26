fn  add_one_v1   (x: u32) -> u32 { x + 1 }

fn main() {
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x|             { x + 1 };
    let add_one_v4 = |x|               x + 1  ;

    assert_eq!(add_one_v1(5), add_one_v2(5));
    assert_eq!(add_one_v3(5), add_one_v4(5));

    let ptr: fn(u32) -> u32 = add_one_v1;
    let closure: fn(u32) -> u32 = |x| x + 1;

    assert_eq!(ptr(10), closure(10));
}