
use core::ffi::{CStr, c_int, c_char, c_void};

extern "C" { 
    pub fn c_func(num: c_int) ->  *const c_char;
    pub fn times2(num: *mut c_int) -> c_void;
}


#[no_mangle]
pub extern "C" fn rs_func(my_num: &mut i32) {
    // Current value of the number
    println!("[ rs_func()]\tHere is your number: {}", *my_num);
    
    // Call C to double the number (by reference)
    unsafe { times2(my_num) };
    println!("[ rs_func()]\tHere is your number: {}", *my_num);

    // Get result from C and convert to Rust usable
    // let local_num: i32 = ;
    let result: &CStr = unsafe { CStr::from_ptr(c_func(*my_num as c_int)) };
    let rust_string: &str = result.to_str().unwrap();

    // Print the Result from C to stdio
    println!("[ rs_func()]\t{}", rust_string);
}