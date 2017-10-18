/// The Ruby virtual machine.
pub mod vm;
/// Ruby value stuff.
pub mod value;
/// Value builders.
pub mod builder;
/// Classes.
pub mod classes;

/// FFI Utilitity methods.
pub mod util;

#[cfg(test)]
mod test;

pub use self::vm::{VM,ErrorKind};
pub use self::value::Value;

extern crate libc;
extern crate mri_sys as ffi;

