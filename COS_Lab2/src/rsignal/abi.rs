
use ::signal::*;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn CreateSignal() -> *const Signal {
    let a = 21_f32 % 7.0 + 7.0;
    let f = 11_f32 % 7.0 + 7.0;
    unsafe { ::std::mem::transmute(Box::new(Signal::new(a, f))) }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn DeleteSignal(ptr: *const Signal) {
    let _: Box<Signal> = unsafe { ::std::mem::transmute(ptr) };
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Graph0(ptr: *const Signal, callback: fn(f32, f32)) {
    let signal: &mut Signal = unsafe { ::std::mem::transmute(ptr) };
    signal.graph0(callback);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Graph1(ptr: *const Signal, callback: fn(f32, f32)) {
    let signal: &mut Signal = unsafe { ::std::mem::transmute(ptr) };
    signal.graph1(callback);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Graph2(ptr: *const Signal, callback: fn(f32, f32)) {
    let signal: &mut Signal = unsafe { ::std::mem::transmute(ptr) };
    signal.graph2(callback);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Math(ptr: *const Signal) -> f32 {
    let signal: &Signal = unsafe { ::std::mem::transmute(ptr) };
    signal.math()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Disp(ptr: *const Signal) -> f32 {
    let signal: &Signal = unsafe { ::std::mem::transmute(ptr) };
    signal.disp()
}
