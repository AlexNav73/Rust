
#![allow(non_snake_case)]

use ::WordSearcher;
use ::libc::c_char;

fn convert_to_rust_str<'a>(s: *const c_char) -> &'a str {
    let c_str = unsafe {
        assert!(!s.is_null());
        ::std::ffi::CStr::from_ptr(s)
    };
    ::std::str::from_utf8(c_str.to_bytes()).unwrap()
}

fn convert_to_c_str(s: &str) -> *const c_char {
    let r_str = ::std::ffi::CString::new(s).unwrap();
    let ptr = r_str.as_ptr();
    ::std::mem::forget(r_str);
    ptr
}

#[no_mangle]
pub extern "C" fn CreateWordSearcher(s: *const c_char) -> *mut WordSearcher {
    Box::into_raw(Box::new(WordSearcher::new(convert_to_rust_str(s))))
}

#[no_mangle]
pub extern "C" fn DeleteWordSearcher(ptr: *mut WordSearcher) {
    let _: Box<WordSearcher> = unsafe { Box::from_raw(ptr) };
}

#[no_mangle]
pub extern "C" fn Search(ptr: *mut WordSearcher, string: *const c_char) -> *const c_char {
    let ws: &mut WordSearcher = unsafe { &mut *ptr };
    let string = convert_to_rust_str(string);

    match ws.search_string(string) {
        Some(ref s) => convert_to_c_str(s),
        None => convert_to_c_str("No matches found")
    }
}
