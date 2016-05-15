use std::ffi::{CString,CStr};
use libc;

pub fn c_string(s: &str) ->  CString {
    CString::new(s).unwrap()
}

pub fn string(raw: *const libc::c_char) -> String {
    unsafe { CStr::from_ptr(raw).to_str().unwrap().to_owned() }
}
