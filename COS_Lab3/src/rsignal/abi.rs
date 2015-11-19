
use ::chart::*;

use ::std::mem::transmute;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn CreateChart() -> *mut Chart {
    unsafe { transmute(Box::new(Chart::new())) }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn DeleteChart(ptr: *mut Chart) {
    let _: Box<Chart> = unsafe { transmute(ptr) };
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn ShowTestSignal(ptr: *mut Chart, callback: fn(f32)) {
    let c: &mut Chart = unsafe { transmute(ptr) };
    c.show_test_signal(callback);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn ShowRecoveredSignal(ptr: *mut Chart, callback: fn(f32)) {
    let c: &mut Chart = unsafe { transmute(ptr) };
    c.show_recovered_signal(callback);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn ShowPolySignal(ptr: *mut Chart, callback: fn(f32)) {
    let c: &mut Chart = unsafe { transmute(ptr) };
    c.show_poly_signal(callback);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn ShowRecoveredPolySignal(ptr: *mut Chart, callback: fn(f32)) {
    let c: &mut Chart = unsafe { transmute(ptr) };
    c.show_recovered_poly_signal(callback);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn ShowRecoveredPolySignalWithoutFi(ptr: *mut Chart, callback: fn(f32)) {
    let c: &mut Chart = unsafe { transmute(ptr) };
    c.show_recovered_poly_signal_without_fi(callback);
}
