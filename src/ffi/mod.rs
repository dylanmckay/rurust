pub use self::consts::*;
pub use self::value::*;
pub use self::functions::*;
pub use self::statics::*;
pub use self::value_type::ruby_value_type;
pub use self::value_type::ruby_value_type::*;
pub use self::ty::*;

pub mod value;
pub mod value_type;
#[allow(non_upper_case_globals)]
pub mod consts;
pub mod statics;
pub mod functions;
#[allow(non_snake_case)]
pub mod ty;

use libc;

#[repr(C)]
#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub struct ID(libc::uintptr_t);

#[repr(C)]
pub struct RBasic {
    flags: VALUE,
    klass: VALUE,
}

