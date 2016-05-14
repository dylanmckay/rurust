#![feature(plugin)]
#![cfg_attr(test, plugin(stainless))]

pub mod vm;
pub mod value;

pub mod ffi;
pub mod util;

pub use self::vm::VM;
pub use self::value::Value;

extern crate libc;
