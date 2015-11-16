
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
pub extern "C" fn ShowPolyChart(ptr: *mut Chart, callback: fn(f32)) {
    let c: &mut Chart = unsafe { transmute(ptr) };
    c.show_poly_signal(callback);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn ShowSpectorChart(ptr: *mut Chart, callback: fn(f32)) {
    let c: &mut Chart = unsafe { transmute(ptr) };
    c.show_spectors(callback);
}
