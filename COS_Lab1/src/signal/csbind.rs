
use signal::*;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn CreateSignal() -> *mut Signal {
    let a = 21_f32 % 7.0 + 7.0;
    let f = 11_f32 % 7.0 + 7.0;
    unsafe { ::std::mem::transmute(Box::new(Signal::new(a, f, 0.0_f32))) }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn DeleteSignal(ptr: *mut Signal) {
    let _: Box<Signal> = unsafe { ::std::mem::transmute(ptr) };
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn Graph1(ptr: *const Signal, callback: fn(i32, f32, f32)) {
    let s: &Signal = unsafe { ::std::mem::transmute(ptr) };
    for i in (1_u32..3+1) {
        s.graph1(i as i32, i as f32, callback);
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn Graph2(ptr: *const Signal, callback: fn(u32, f32, f32)) {
    let s: &Signal = unsafe { ::std::mem::transmute(ptr) };
    for i in (1_u32..3+1) {
        s.graph2(i, i as f32, callback);
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn Graph3(ptr: *const Signal, callback: fn(u32, f32, f32)) {
    let s: &Signal = unsafe { ::std::mem::transmute(ptr) };
    s.graph3(callback);
}

