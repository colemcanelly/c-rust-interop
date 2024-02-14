
extern "C" { pub fn c_func(); }

// use std::os::raw::{c_int, c_char, c_void};

#[no_mangle]
pub extern "C" fn rs_func() {
    println!("[rust] Hello from rs_func()");
    unsafe { c_func(); }
}