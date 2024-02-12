use std::ffi::CString;
use std::os::raw::c_char;

extern "C" {
    fn puts(s: *const c_char);
}

fn main() {
    let c_str = CString::new("RustaCean!").expect("?!");

    //cross language call to C
    unsafe {
        puts(c_str.as_ptr());
    }
}