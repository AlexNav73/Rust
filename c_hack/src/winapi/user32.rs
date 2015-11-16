
extern crate libc;
use std::ffi::CString;
use std::ptr;

type HWND = *const libc::c_void;
type LPCTSTR = *const libc::c_char;
type UINT = libc::c_uint;

#[link(name = "user32")]
extern "system" {
    #[allow(non_snake_case)]
    pub fn MessageBoxA(hWnd: HWND, lpText: LPCTSTR, lpCaption: LPCTSTR, uType: UINT) -> UINT;
}

#[allow(non_snake_case)]
pub fn MessageBox(text: &str, caption: &str, u_type: u32) {
    let c_text = CString::new(text).unwrap();
    let c_caption = CString::new(caption).unwrap();
    unsafe { MessageBoxA(ptr::null(), c_text.as_ptr(), c_caption.as_ptr(), u_type); }
}


