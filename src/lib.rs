#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(stainless))]

/// The Ruby virtual machine.
pub mod vm;
/// Ruby value stuff.
pub mod value;
/// Value builders.
pub mod builder;

/// FFI Utilitity methods.
pub mod util;

pub use self::vm::{VM,ErrorKind};
pub use self::value::Value;

extern crate libc;
extern crate mri_sys as ffi;

