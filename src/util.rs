use std::ffi::CString;
use libc;

pub fn c_string(s: &str) ->  CString {
    CString::new(s).unwrap()
}

pub fn string(raw: *const libc::c_char) -> String {
    unsafe {
        CString::from_raw(raw as *mut libc::c_char).into_string().unwrap()
    }
}
