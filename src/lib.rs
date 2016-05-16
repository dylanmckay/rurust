#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(stainless))]

pub mod vm;
pub mod value;
pub mod builder;

pub mod util;

pub use self::vm::{VM,ErrorKind};
pub use self::value::Value;

extern crate libc;
extern crate mri_sys as ffi;

