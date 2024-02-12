fn main() {
    let mut num: f32 = 3.1415;
 
    let raw_const = &num as *const f32;
    let raw_mut = &mut num as *mut f32;

    unsafe {
        //unsafe to dereference the pointer because it could have changed
        *raw_mut = 1.414;
        println!("raw_const = {}", *raw_const);
    }
}