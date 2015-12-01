
use ::std::mem::*;
use ::signal::*;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn CreateSignal(b1: f64, b2: f64) -> *mut Signal {
    unsafe { transmute(Box::new(Signal::new(b1, b2))) }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn DeleteSignal(ptr: *mut Signal) {
    let _: Box<Signal> = unsafe { transmute(ptr) };
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn ShowSignal(ptr: *const Signal, callback: fn(f64)) {
    let s: &Signal = unsafe { transmute(ptr) };
    s.show_signal(callback);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn ParabolaAliasing(ptr: *const Signal, callback: fn(f64)) {
    let s: &Signal = unsafe { transmute(ptr) };
    ::algos::parabola_aliasing(s, callback);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn WindowAliasing(ptr: *const Signal, callback: fn(f64)) {
    let s: &Signal = unsafe { transmute(ptr) };
    ::algos::window_aliasing(s, callback);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn MedianAliasing(ptr: *const Signal, callback: fn(f64)) {
    let s: &Signal = unsafe { transmute(ptr) };
    ::algos::median_aliasing(s, callback);
}


