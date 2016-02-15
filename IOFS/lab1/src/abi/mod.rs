
use ::WordCounter;
use ::libc::c_char;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn CreateWordCounter(s: *const c_char) -> *mut WordCounter {
    let c_str = unsafe {
        assert!(!s.is_null());
        ::std::ffi::CStr::from_ptr(s)
    };
    let r_str = ::std::str::from_utf8(c_str.to_bytes()).unwrap();
    Box::into_raw(Box::new(WordCounter::new(r_str)))
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn DeleteWordCounter(ptr: *mut WordCounter) {
    let _: Box<WordCounter> = unsafe { Box::from_raw(ptr) };
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Enumerate(ptr: *mut WordCounter, f: fn(u32, f32)) {
    let wc: &mut WordCounter = unsafe { ::std::mem::transmute(ptr) };
    let v: Vec<(&str, f32)> = wc.count_probability();

    for (i, &(_, v)) in v.iter().enumerate() {
        f(i as u32, v);
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn CountConst(ptr: *mut WordCounter, f: fn(f32)) {
    let wc: &mut WordCounter = unsafe { ::std::mem::transmute(ptr) };
    let v: Vec<(&str, f32)> = wc.count_probability();

    for (i, &(_, v)) in v.iter().enumerate() {
        f((i + 1) as f32 * v);
    }
}

