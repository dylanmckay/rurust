#![feature(libc)]

use std::mem;

extern crate libc;

fn main() {
    println!("cargo:rustc-link-lib=dylib=ruby");

    if mem::size_of::<libc::uintptr_t>() >= mem::size_of::<f64>() {
        println!("cargo:rustc-cfg=mri_use_flonum");
    }
}

