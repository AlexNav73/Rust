
#[no_mangle]
pub extern "C" fn mul(a: i32, b: i32) -> i32 {
    a * b
}

#[no_mangle]
pub extern "C" fn do_not_call_this_fn() {
    panic!("I told you!");
}
