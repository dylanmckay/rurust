use super::VALUE;
use libc;

pub const Qfalse: VALUE = VALUE(0x00);

#[cfg(mri_use_flonum)]
pub const Qtrue:  VALUE = VALUE(0x14);
#[cfg(not(mri_use_flonum))]
pub const Qtrue:  VALUE = VALUE(0x02);

#[cfg(mri_use_flonum)]
pub const Qnil:   VALUE = VALUE(0x08);
#[cfg(not(mri_use_flonum))]
pub const Qnil:   VALUE = VALUE(0x04);

#[cfg(mri_use_flonum)]
pub const Qundef: VALUE = VALUE(0x34);
#[cfg(not(mri_use_flonum))]
pub const Qundef: VALUE = VALUE(0x06);

#[cfg(mri_use_flonum)]
pub const IMMEDIATE_MASK: VALUE = VALUE(0x07);
#[cfg(not(mri_use_flonum))]
pub const IMMEDIATE_MASK: VALUE = VALUE(0x03);

pub const FIXNUM_FLAG:    VALUE = VALUE(0x01);

#[cfg(mri_use_flonum)]
pub const FLONUM_MASK:    VALUE = VALUE(0x03);
#[cfg(not(mri_use_flonum))]
pub const FLONUM_MASK:    VALUE = VALUE(0x00);

pub const FLONUM_FLAG:    VALUE = VALUE(0x02);

#[cfg(mri_use_flonum)]
pub const SYMBOL_FLAG:    VALUE = VALUE(0x0c);
#[cfg(not(mri_use_flonum))]
pub const SYMBOL_FLAG:    VALUE = VALUE(0x0e);

pub const SPECIAL_SHIFT: libc::uintptr_t = 8;
