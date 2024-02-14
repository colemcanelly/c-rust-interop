
extern "C" { pub fn c_func(); }


#[no_mangle]
pub extern "C" fn rs_func() {
    println!("[rust] Hello from rs_func()");
    unsafe { c_func(); }
}